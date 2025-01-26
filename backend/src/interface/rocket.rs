use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::Build;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sqlx::postgres::PgPoolOptions;
use std::env;

use super::handlers::streams::TaskStateChannel;
use super::routes;

// Define the Rocket instance
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

    // Check if the DATABASE_URL environment variable is set
    let _ = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Return the Rocket instance
    rocket::build()
        // .attach(Db::fairing())
        .configure(rocket::Config::figment().merge(("port", 9797)))
        .attach(AdHoc::on_ignite("Database", |rocket| async {
            let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to create pool.");

            rocket.manage(pool)
        }))
        .manage(TaskStateChannel::new())
        .mount("/api", routes::get_routes())
        .attach(cors)
}
