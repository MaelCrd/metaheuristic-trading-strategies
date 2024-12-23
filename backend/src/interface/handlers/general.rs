use rocket::get;
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
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
