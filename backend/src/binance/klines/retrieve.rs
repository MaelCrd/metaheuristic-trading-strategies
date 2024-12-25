use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use sqlx::Row;

use super::acquire;
use super::utils;
use crate::objects::objects::CryptoInterval;
use crate::objects::objects::Kline;
use crate::objects::objects::KlineCollection;

// Function to retrieve klines from database with the given parameters
pub async fn retrieve_klines_simple(
    klines_collection: &mut KlineCollection,
    symbol: &str,
    interval: &CryptoInterval,
    limit_minutes: i64,
    training_percentage: f64,
    force_fetch: bool,
) -> Result<(), sqlx::Error> {
    retrieve_klines(
        klines_collection,
        symbol,
        interval,
        limit_minutes,
        training_percentage,
        force_fetch,
        None,
        None,
    )
    .await
}

// Function to retrieve klines from database
pub async fn retrieve_klines(
    klines_collection: &mut KlineCollection,
    symbol: &str,
    interval: &CryptoInterval,
    limit_minutes: i64,
    training_percentage: f64,
    force_fetch: bool,
    only_before: Option<DateTime<Utc>>,
    real_limit_n: Option<i32>,
) -> Result<(), sqlx::Error> {
    // Retrieve the klines from the database
    let table_name = utils::get_table_name(symbol, interval);
    let mut limit = limit_minutes / interval.to_minutes();

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
    if !table_exists || table_length == 0 || table_length < limit || force_fetch {
        // Acquire the klines from the Binance API
        acquire::acquire_klines(
            &pool,
            &symbol,
            &interval,
            &Some(limit),
            table_exists,
            &mut table_length,
            force_fetch,
        )
        .await
        .unwrap();
    }

    let before_cond = match only_before {
        Some(only_before) => format!("\n\rWHERE open_time <= {}", only_before.timestamp_millis()),
        None => "\n\r".to_string(),
    };
    if Some(only_before) != None && real_limit_n != None {
        limit = real_limit_n.unwrap() as i64;
    }

    // Query the database
    let result = sqlx::query(&format!(
        r#"
        SELECT open_time, open, high, low, close, volume, close_time, quote_asset_volume, number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume
        FROM {}{}
        ORDER BY open_time DESC
        LIMIT {}
        "#,
        table_name, before_cond, limit
    )).fetch_all(&pool).await.unwrap();

    // Parse the results and add them to the klines collection
    let training_length = (result.len() as f64 * training_percentage).round() as usize;
    let mut i = 0;
    for row in result.iter().rev() {
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

    klines_collection.symbol = symbol.to_string();
    klines_collection.interval = interval.clone();

    Ok(())
}

pub async fn retrieve_klines_extend(
    klines_collection_model: &KlineCollection,
    klines_collection_extended: &mut KlineCollection,
    additional_klines: i32,
) -> Result<(), sqlx::Error> {
    // Calculate the new limit_minutes
    let last_open_time = klines_collection_model
        .get_last_open_time()
        .timestamp_millis();
    let new_limit_minutes = klines_collection_model.get_length() as i64 * klines_collection_model.interval.to_minutes()
        + (Utc::now().timestamp_millis() - last_open_time) / 60000  // Difference between the last open time and now
        + klines_collection_model.interval.to_minutes() * additional_klines as i64; // Additional klines to the limit

    println!(
        "Difference between the last open time and now: {}",
        (Utc::now().timestamp_millis() - last_open_time) / 60000
    );
    println!(
        "Additional klines to the limit: {}",
        klines_collection_model.interval.to_minutes() * additional_klines as i64
    );

    klines_collection_model.display();

    println!("New limit minutes: {}", new_limit_minutes);

    // Retrieve the klines from the database
    retrieve_klines(
        klines_collection_extended,
        &klines_collection_model.symbol,
        &klines_collection_model.interval,
        new_limit_minutes,
        klines_collection_model.training_percentage,
        false,
        Some(klines_collection_model.get_last_open_time()),
        Some(klines_collection_model.get_length() + additional_klines),
    )
    .await;

    klines_collection_extended.display();

    Ok(())
}
