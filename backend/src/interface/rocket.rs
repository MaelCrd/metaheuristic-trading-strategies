use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{get, post, routes};
use rocket::{http::Method, Build, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

use crate::interface::objects::{CreateCryptoObject, CryptoObject, Status};

#[get("/health")]
async fn health_check() -> Json<Status> {
    Json(Status {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// Define a route to create a new crypto object
#[post(
    "/crypto_objects",
    format = "application/json",
    data = "<create_crypto_object>"
)]
async fn create_crypto_object(
    pool: &State<PgPool>,
    create_crypto_object: Json<CreateCryptoObject>,
) -> Result<Json<Vec<CryptoObject>>, String> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO CryptoObject (name, type)
        VALUES ($1, $2)
        RETURNING id, name, type
        "#,
        create_crypto_object.name,
        create_crypto_object.r#type,
    )
    .fetch_one(&**pool)
    .await;

    match rec {
        // Ok(row) => Ok(Json(CryptoObject {
        //     id: row.id,
        //     name: row.name,
        //     r#type: row.r#type,
        // })),
        Ok(_) => Ok(get_crypto_objects(pool).await.unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

// Define a route to get all crypto objects
#[get("/crypto_objects")]
async fn get_crypto_objects(pool: &State<PgPool>) -> Result<Json<Vec<CryptoObject>>, String> {
    let recs = sqlx::query!(
        r#"
        SELECT id, name, type
        FROM CryptoObject
        "#,
    )
    .fetch_all(&**pool)
    .await;

    match recs {
        Ok(rows) => {
            let crypto_objects: Vec<CryptoObject> = rows
                .into_iter()
                .map(|row| CryptoObject {
                    id: row.id,
                    name: row.name,
                    r#type: row.r#type,
                })
                .collect();
            Ok(Json(crypto_objects))
        }
        Err(e) => Err(e.to_string()),
    }
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
            routes![health_check, create_crypto_object, get_crypto_objects],
        )
        .attach(cors)
}
