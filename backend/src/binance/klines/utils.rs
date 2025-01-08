use sqlx::PgPool;

use crate::objects::{intervals::CryptoInterval, klines::KlineCollection};

pub fn get_table_name(symbol: &str, interval: &CryptoInterval) -> String {
    format!("klines_{}_{}", symbol, interval.to_string()).to_lowercase()
}

pub fn get_table_name_collection(klines_collection: &KlineCollection) -> String {
    get_table_name(
        &klines_collection.symbol.symbol,
        &klines_collection.interval,
    )
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

pub async fn check_column_exists(pool: &PgPool, table_name: &str, column_name: &str) -> bool {
    sqlx::query_scalar(&format!(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.columns 
            WHERE table_name = '{}' AND column_name = '{}'
        )
        "#,
        table_name, column_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to check if column exists")
}

pub async fn check_columns_exists(
    pool: &PgPool,
    table_name: &str,
    column_names: &Vec<String>,
) -> bool {
    let mut exists = true;

    for column_name in column_names {
        exists = check_column_exists(pool, table_name, column_name).await;

        if !exists {
            break;
        }
    }

    exists
}

pub async fn create_indicator_column(
    pool: &PgPool,
    table_name: &str,
    column_name: &str,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query(&format!(
        r#"
        ALTER TABLE {} ADD COLUMN {} FLOAT
        "#,
        table_name, column_name
    ))
    .execute(pool)
    .await;

    result.map(|_| ())
}

pub async fn create_indicator_columns(
    pool: &PgPool,
    table_name: &str,
    column_names: &Vec<String>,
) -> Result<(), sqlx::Error> {
    for column_name in column_names {
        create_indicator_column(pool, table_name, column_name)
            .await
            .unwrap();
    }

    Ok(())
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

// Check KlineCollection klines integrity
pub fn check_klines_collection_integrity(klines_collection: &KlineCollection) -> bool {
    // Create an iterator over all klines
    let all_klines_iter = klines_collection
        .past
        .iter()
        .chain(klines_collection.training.iter())
        .chain(klines_collection.validation.iter());

    // Get the interval in seconds
    let interval_sec = klines_collection.interval.to_minutes() * 60;

    // Check if all klines are in the correct order
    let check_1 = all_klines_iter
        .clone()
        .zip(all_klines_iter.clone().skip(1))
        .all(|(kline1, kline2)| kline1.open_time < kline2.open_time);

    let check_2 = all_klines_iter
        .clone()
        .zip(all_klines_iter.clone().skip(1))
        .all(|(kline1, kline2)| {
            kline2.open_time.timestamp() - kline1.open_time.timestamp() == interval_sec
        });

    // Print all klines (debug)
    // println!("Past klines:");
    // for kline in klines_collection.past.iter() {
    //     println!("{:?}", kline.open_time);
    // }
    // println!("Training klines:");
    // for kline in klines_collection.training.iter() {
    //     println!("{:?}", kline.open_time);
    // }
    // println!("Validation klines:");
    // for kline in klines_collection.validation.iter() {
    //     println!("{:?}", kline.open_time);
    // }

    check_1 && check_2
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
