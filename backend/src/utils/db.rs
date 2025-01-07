use sqlx::PgPool;
use std::env;

pub async fn get_new_pool() -> PgPool {
    PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool.")
}
