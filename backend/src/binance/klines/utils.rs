use std::env;

use sqlx::PgPool;

use crate::objects::objects::{CryptoInterval, Kline, KlineCollection};

pub fn get_table_name(symbol: &str, interval: &CryptoInterval) -> String {
    format!("klines_{}_{}", symbol, interval.to_string()).to_lowercase()
}

pub async fn connect_to_db() -> PgPool {
    PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool.")
}

pub async fn create_klines_table(pool: &PgPool, table_name: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query(&format!(
        r#"
        CREATE TABLE {} (
            open_time BIGINT PRIMARY KEY,
            open FLOAT NOT NULL,
            high FLOAT NOT NULL,
            low FLOAT NOT NULL,
            close FLOAT NOT NULL,
            volume FLOAT NOT NULL,
            close_time BIGINT NOT NULL,
            quote_asset_volume FLOAT NOT NULL,
            number_of_trades BIGINT NOT NULL,
            taker_buy_base_asset_volume FLOAT NOT NULL,
            taker_buy_quote_asset_volume FLOAT NOT NULL
        )
        "#,
        table_name
    ))
    .execute(pool)
    .await;

    result.map(|_| ())
}

pub async fn check_table_exists(pool: &PgPool, table_name: &str) -> bool {
    sqlx::query_scalar(&format!(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_name = '{}'
        )
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to check if table exists")
}

pub async fn get_table_length(pool: &PgPool, table_name: &str) -> i64 {
    sqlx::query_scalar(&format!(
        r#"
        SELECT COUNT(*) FROM {}
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get table length")
}

pub async fn get_min_open_time(pool: &PgPool, table_name: &str) -> u64 {
    let result: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT MIN(open_time) FROM {}
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get min open time");

    result.abs() as u64
}

pub async fn get_max_open_time(pool: &PgPool, table_name: &str) -> u64 {
    let result: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT MAX(open_time) FROM {}
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get max open time");

    result.abs() as u64
}

// &serde_json::Value to f64
pub fn json_value_to_f64(value: &serde_json::Value) -> f64 {
    value
        .as_str()
        .expect("Failed to get value")
        .parse::<f64>()
        .unwrap()
}

// pub async fn check_table_empty(pool: &PgPool, table_name: &str) -> bool {
//     let result: bool = sqlx::query_scalar(&format!(
//         r#"
//         SELECT EXISTS (
//             SELECT FROM {}
//         )
//         "#,
//         table_name
//     ))
//     .fetch_one(pool)
//     .await
//     .expect("Failed to check if table is empty");

//     !result
// }
