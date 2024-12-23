use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;

use crate::objects::objects::Task;
use crate::utils;

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
            SELECT id, state, created_at, other_parameters, mh_object_id, crypto_list_id, result_id
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
                state: row.state,
                created_at: utils::datetime::convert_primitive_to_chrono(row.created_at),
                other_parameters: row.other_parameters,
                mh_object_id: row.mh_object_id,
                crypto_list_id: row.crypto_list_id,
                result_id: row.result_id,
            })
            .collect();

        // Return the task
        Ok(Json(tasks))
    } else {
        let recs = sqlx::query!(
            r#"
            SELECT id, state, created_at, other_parameters, mh_object_id, crypto_list_id, result_id
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
                state: row.state,
                created_at: utils::datetime::convert_primitive_to_chrono(row.created_at),
                other_parameters: row.other_parameters,
                mh_object_id: row.mh_object_id,
                crypto_list_id: row.crypto_list_id,
                result_id: row.result_id,
            })
            .collect();

        // Return the tasks
        Ok(Json(tasks))
    }
}
