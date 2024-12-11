use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{get, post, routes};
use rocket::{http::Method, Build, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

use crate::interface::objects::{CreateCryptoList, CryptoList, Status};

#[get("/health")]
async fn health_check() -> Json<Status> {
    Json(Status {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// Define a route to create a new crypto list
#[post(
    "/crypto_lists",
    format = "application/json",
    data = "<create_crypto_list>"
)]
async fn create_crypto_list(
    pool: &State<PgPool>,
    create_crypto_list: Json<CreateCryptoList>,
) -> Json<Vec<CryptoList>> {
    let _ = sqlx::query!(
        r#"
        INSERT INTO crypto_list (name, type)
        VALUES ($1, $2)
        RETURNING id, name, type
        "#,
        create_crypto_list.name,
        create_crypto_list.r#type,
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    get_crypto_lists(pool).await
}

// Define a route to get all crypto lists
#[get("/crypto_lists")]
async fn get_crypto_lists(pool: &State<PgPool>) -> Json<Vec<CryptoList>> {
    let recs = sqlx::query!(
        r#"
        SELECT id, name, type
        FROM crypto_list
        "#,
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let crypto_lists: Vec<CryptoList> = recs
        .into_iter()
        .map(|row| CryptoList {
            id: row.id,
            name: row.name,
            r#type: row.r#type,
        })
        .collect();

    Json(crypto_lists)
}

// ------------------------------------------------

// #[launch]
pub fn rocket() -> rocket::Rocket<Build> {
    // Configure CORS
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration error");

    let _ = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::build()
        // .attach(Db::fairing())
        .configure(rocket::Config::figment().merge(("port", 9797)))
        .attach(AdHoc::on_ignite("Database", |rocket| async {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to create pool.");

            rocket.manage(pool)
        }))
        .mount(
            "/api",
            routes![health_check, create_crypto_list, get_crypto_lists],
        )
        .attach(cors)
}
