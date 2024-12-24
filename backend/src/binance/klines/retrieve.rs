use chrono::DateTime;
use chrono::Utc;
use sqlx::Row;

use super::acquire;
use super::utils;
use crate::objects::objects::CryptoInterval;
use crate::objects::objects::Kline;
use crate::objects::objects::KlineCollection;

// Function to retrieve klines from database
pub async fn retrieve_klines(
    klines_collection: &mut KlineCollection,
    symbol: &str,
    interval: &CryptoInterval,
    limit_minutes: i64,
    training_percentage: f64,
) -> Result<(), sqlx::Error> {
    // Retrieve the klines from the database
    let table_name = utils::get_table_name(symbol, interval);
    let limit = limit_minutes / interval.to_minutes();

    // Check if the table exists
    let pool = utils::connect_to_db().await;
    let table_exists = utils::check_table_exists(&pool, &table_name).await;
    let mut table_length = match table_exists {
        true => utils::get_table_length(&pool, &table_name).await,
        false => 0,
    };
    // let mut table_empty = true;
    // if table_exists {
    //     table_empty = utils::check_table_empty(pool, &table_name).await;
    // }

    // If the table does not exist or is empty, fetch the klines from the Binance API
    if !table_exists || table_length == 0 || table_length < limit {
        // Acquire the klines from the Binance API
        acquire::acquire_klines(
            &pool,
            &symbol,
            &interval,
            &Some(limit),
            table_exists,
            &mut table_length,
        )
        .await
        .unwrap();
    }

    // Query the database
    let result = sqlx::query(&format!(
        r#"
        SELECT open_time, open, high, low, close, volume, close_time, quote_asset_volume, number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume
        FROM {}
        ORDER BY open_time DESC
        LIMIT {}
        "#,
        table_name, limit
    )).fetch_all(&pool).await.unwrap();

    // Parse the results and add them to the klines collection
    let training_length = (result.len() as f64 * training_percentage).round() as usize;
    let mut i = 0;
    for row in result {
        if i < training_length {
            &mut klines_collection.training
        } else {
            &mut klines_collection.validation
        }
        .push(Kline {
            open_time: DateTime::<Utc>::from_timestamp_millis(row.get::<i64, _>("open_time"))
                .unwrap(),
            open: row.get("open"),
            high: row.get("high"),
            low: row.get("low"),
            close: row.get("close"),
            volume: row.get("volume"),
            close_time: DateTime::<Utc>::from_timestamp_millis(row.get::<i64, _>("close_time"))
                .unwrap(),
            quote_asset_volume: row.get("quote_asset_volume"),
            number_of_trades: row.get("number_of_trades"),
            taker_buy_base_asset_volume: row.get("taker_buy_base_asset_volume"),
            taker_buy_quote_asset_volume: row.get("taker_buy_quote_asset_volume"),
        });
        i += 1;
    }

    Ok(())
}
