use rocket::serde::json::Json;
use rocket::{get, post};
use rocket::{put, State};
use sqlx::PgPool;

use crate::objects::objects::{CreateTask, Task, TaskState};
use crate::utils;

use super::streams::{TaskStateChannel, TaskUpdate};

// Define a route to get all tasks
#[get("/task?<id>")]
pub async fn get_tasks(
    pool: &State<PgPool>,
    id: Option<String>,
) -> Result<Json<Vec<Task>>, rocket::http::Status> {
    if let Some(id) = id {
        let id = id.parse::<i32>().unwrap();
        let recs = sqlx::query!(
            r#"
            SELECT id, state::TEXT as state, created_at, other_parameters, mh_object_id, crypto_list_id, indicator_combination_id, result_id
            FROM task
            WHERE id = $1
            "#,
            id
        )
        .fetch_all(&**pool)
        .await
        .unwrap();

        // Check if the task was found
        if recs.is_empty() {
            println!("Task not found (id: {})", id);
            return Err(rocket::http::Status::NotFound);
        }

        let tasks: Vec<Task> = recs
            .into_iter()
            .map(|row| Task {
                id: row.id,
                state: TaskState::parse_from(row.state.as_deref().unwrap()),
                created_at: utils::datetime::convert_primitive_to_chrono(row.created_at),
                other_parameters: row.other_parameters,
                mh_object_id: row.mh_object_id,
                crypto_list_id: row.crypto_list_id,
                indicator_combination_id: row.indicator_combination_id,
                result_id: row.result_id,
            })
            .collect();

        // Return the task
        Ok(Json(tasks))
    } else {
        let recs = sqlx::query!(
            r#"
            SELECT id, state::TEXT as state, created_at, other_parameters, mh_object_id, crypto_list_id, indicator_combination_id, result_id
            FROM task
            "#,
        )
        .fetch_all(&**pool)
        .await
        .unwrap();

        let tasks: Vec<Task> = recs
            .into_iter()
            .map(|row| Task {
                id: row.id,
                state: TaskState::parse_from(row.state.as_deref().unwrap()),
                created_at: utils::datetime::convert_primitive_to_chrono(row.created_at),
                other_parameters: row.other_parameters,
                mh_object_id: row.mh_object_id,
                crypto_list_id: row.crypto_list_id,
                indicator_combination_id: row.indicator_combination_id,
                result_id: row.result_id,
            })
            .collect();

        // Return the tasks
        Ok(Json(tasks))
    }
}

// Define a route to create a task
#[post("/task?<queue>", data = "<task>")]
pub async fn create_task(
    pool: &State<PgPool>,
    task: Json<CreateTask>,
    queue: Option<bool>,
) -> Result<Json<Vec<Task>>, rocket::http::Status> {
    let res = sqlx::query!(
        r#"
        INSERT INTO task (other_parameters, mh_object_id, crypto_list_id, indicator_combination_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        task.other_parameters,
        task.mh_object_id,
        task.crypto_list_id,
        task.indicator_combination_id
    )
    .fetch_all(&**pool)
    .await;

    if res.is_err() {
        println!("Task creation failed: {:?}", res.err());
        return Err(rocket::http::Status::InternalServerError);
    }
    let res_unwrapped = res.unwrap();

    if queue.unwrap_or(false) {
        let id = res_unwrapped[0].id;

        queue_task(pool, id).await.unwrap();

        // Check if the task was found
        // if result.rows_affected() == 0 {
        //     println!("Task not found or already queued (id: {})", id);
        //     return Err(rocket::http::Status::NotFound);
        // }
    }

    // Return the task
    Ok(get_tasks(pool, None).await.unwrap())
}

// Define a route to queue a task
#[put("/task/queue?<id>")]
pub async fn queue_task(
    pool: &State<PgPool>,
    id: i32,
) -> Result<Json<Vec<Task>>, rocket::http::Status> {
    let result = sqlx::query!(
        r#"
        UPDATE task
        SET state = 'PENDING'
        WHERE id = $1 AND state = 'CREATED'
        "#,
        id
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Check if the task was found
    if result.rows_affected() == 0 {
        println!("Task not found or already queued (id: {})", id);
        return Err(rocket::http::Status::NotFound);
    }

    // Return the task
    Ok(get_tasks(pool, None).await.unwrap())
}

// Define a route to cancel a task
#[put("/task/cancel?<id>")]
pub async fn cancel_task(
    pool: &State<PgPool>,
    id: i32,
) -> Result<Json<Vec<Task>>, rocket::http::Status> {
    let result = sqlx::query!(
        r#"
        UPDATE task
        SET state = 'CANCELLING'
        WHERE id = $1 AND (state = 'CREATED' OR state = 'PENDING' OR state = 'RUNNING')
        "#,
        id
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Check if the task was found
    if result.rows_affected() == 0 {
        println!("Task not found or already cancelled (id: {})", id);
        return Err(rocket::http::Status::NotFound);
    }

    // Return the task
    Ok(get_tasks(pool, None).await.unwrap())
}

// Not routes, but functions to be used by the manager
pub async fn update_task_state(
    pool: &State<PgPool>,
    channel: &TaskStateChannel,
    id: i32,
    state: TaskState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = sqlx::query!(
        r#"
        UPDATE task
        SET state = $2
        WHERE id = $1 AND state != $2
        "#,
        id,
        state.clone() as TaskState
    )
    .execute(&**pool)
    .await
    .unwrap();

    match result.rows_affected() {
        0 => {
            println!(
                "Task not found or already in the desired state (id: {})",
                id
            );
            Err("Task not found or already in the desired state".into())
        }
        _ => {
            let _ = channel.sender.send(TaskUpdate {
                task_id: id,
                state: state.convert_to(),
            });
            println!(
                "Sent task update to channel (id: {:?}, state: {:?})",
                id, state
            );
            Ok(())
        }
    }
}
