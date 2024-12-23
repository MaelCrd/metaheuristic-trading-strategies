use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;

use crate::objects::objects::Task;
use crate::utils;

// Define a route to get all tasks
#[get("/task")]
pub async fn get_tasks(pool: &State<PgPool>) -> Json<Vec<Task>> {
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

    Json(tasks)
}
