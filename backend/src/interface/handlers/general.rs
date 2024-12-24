use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
use rocket::{delete, get, State};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tokio::time::interval; // self,

use crate::objects::objects::Status;

#[get("/health")]
pub async fn health_check() -> Json<Status> {
    Json(Status {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Produce an infinite series of `"hello"`s, one per second.
#[get("/infinite-hellos")]
pub fn hello() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            yield "hello\n";
            interval.tick().await;
        }
    }
}

// Define a route to purge all crypto lists, MH objects, and tasks that are hidden and have no dependencies
#[delete("/purge-hidden-orphans")]
pub async fn purge_hidden_orphans(pool: &State<PgPool>) -> rocket::http::Status {
    // Remove all hidden crypto lists that have no dependencies
    let _ = sqlx::query!(
        r#"
        DELETE FROM crypto_list
        WHERE hidden = TRUE
        AND id NOT IN (
            SELECT DISTINCT crypto_list_id
            FROM task
        )
        AND id NOT IN (
            SELECT DISTINCT crypto_list_id
            FROM crypto_list_x_crypto_symbol
        )
        "#
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Remove all hidden MH objects that have no dependencies
    let _ = sqlx::query!(
        r#"
        DELETE FROM mh_object
        WHERE hidden = TRUE
        AND id NOT IN (
            SELECT DISTINCT mh_object_id
            FROM task
        )
        "#
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Remove all tasks that have no mh_object_id and crypto_list_id
    let _ = sqlx::query!(
        r#"
        DELETE FROM task
        WHERE mh_object_id IS NULL
        AND crypto_list_id IS NULL
        "#
    )
    .execute(&**pool)
    .await
    .unwrap();

    rocket::http::Status::Ok
}
