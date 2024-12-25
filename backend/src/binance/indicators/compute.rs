use sqlx::PgPool;

use super::super::klines::utils;
use crate::objects::indicators::IndicatorTrait;
use crate::objects::{indicators::Indicator, objects::KlineCollection};

pub async fn compute_indicator(
    _pool: &PgPool,
    indicator: &Indicator,
    kline_collection: &KlineCollection,
) -> Result<(), sqlx::Error> {
    // We assume klines are already present in the database
    let table_name = utils::get_table_name_collection(&kline_collection);
    let _number_of_rows = kline_collection.get_length();

    // Check if the table exists
    let pool = utils::connect_to_db().await;
    let table_exists = utils::check_table_exists(&pool, &table_name).await;
    let _table_length = match table_exists {
        true => utils::get_table_length(&pool, &table_name).await,
        false => 0,
    };

    // Query the database to retrieve the open_time of rows that are missing the indicator columns
    // and between the first and last open time
    let first_open_time = kline_collection.get_first_open_time();
    let last_open_time = kline_collection.get_last_open_time();
    let result = sqlx::query(&format!(
        r#"
        SELECT open_time
        FROM {}
        WHERE {} IS NULL
        AND open_time BETWEEN {} AND {}
        ORDER BY open_time ASC
        "#,
        table_name,
        indicator.column_names().join(" IS NULL OR "),
        first_open_time.timestamp_millis(),
        last_open_time.timestamp_millis()
    ))
    .fetch_all(&pool)
    .await
    .unwrap();

    let missing_rows = result.len();

    println!("Missing rows: {}", missing_rows);

    Ok(())
}
