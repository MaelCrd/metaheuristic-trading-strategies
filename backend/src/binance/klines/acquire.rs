use chrono::TimeZone;
use reqwest::Client;
use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;

use super::utils;
use crate::{objects::intervals::CryptoInterval, utils::loading};

const BINANCE_FUTURES_API_URL: &str = "https://fapi.binance.com";
const KLINES_LIMIT: &str = "5";
const REQUESTS_DELAY_SEC: u64 = 1;

#[derive(PartialEq)]
enum KlinesFetchType {
    Recent,
    Older,
}

pub async fn acquire_klines(
    pool: &PgPool,
    symbol: &str,
    interval: &CryptoInterval,
    limit: &Option<i64>,
    table_exists: bool,
    table_length: &mut i64,
    force_fetch: bool,
) -> Result<(), sqlx::Error> {
    let table_name = utils::get_table_name(symbol, &interval);
    println!("Table name: {}", table_name);

    // let table_exists = utils::check_table_exists(&pool, &table_name).await;
    // let mut table_length = utils::get_table_length(&pool, &table_name).await;

    println!("Table length: {}", table_length);

    let client = Client::new();

    if !table_exists || *table_length == 0 {
        println!("Table doesn't exist, creating table and fetching historical data...");
        if !table_exists {
            let result = utils::create_klines_table(&pool, &table_name).await;
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }
        loop_fetch_klines(
            &pool,
            &client,
            &table_name,
            symbol,
            interval,
            table_length,
            limit,
            &KlinesFetchType::Older,
            false,
        )
        .await;
    } else {
        println!("Table exists, fetching recent and older data...");
        loop_fetch_klines(
            &pool,
            &client,
            &table_name,
            symbol,
            interval,
            table_length,
            limit,
            &KlinesFetchType::Recent,
            force_fetch,
        )
        .await;
        loop_fetch_klines(
            &pool,
            &client,
            &table_name,
            symbol,
            interval,
            table_length,
            limit,
            &KlinesFetchType::Older,
            false,
        )
        .await;
    }

    // Check the integrity of the klines
    check_klines_integrity(&pool, &table_name).await;

    Ok(())
}

async fn loop_fetch_klines(
    pool: &PgPool,
    client: &Client,
    table_name: &str,
    symbol: &str,
    interval: &CryptoInterval,
    table_length: &mut i64,
    limit: &Option<i64>,
    fetch_type: &KlinesFetchType,
    force_fetch: bool,
) {
    let time_string: &str;
    let mut time_param: u64;
    if *fetch_type == KlinesFetchType::Recent {
        time_string = "startTime";
        time_param = utils::get_max_open_time(pool, table_name).await;
    } else {
        time_string = "endTime";
        time_param = match *table_length {
            0 => {
                chrono::Utc::now().timestamp_millis() as u64
                    + chrono::Duration::days(100).num_milliseconds() as u64
            }
            _ => utils::get_min_open_time(pool, table_name).await,
        };
    }

    loop {
        // If the limit is reached, stop fetching data
        if let Some(limit) = limit {
            loading::print_loading_progress(*table_length as i32, *limit as i32);
            if !force_fetch && *table_length >= *limit {
                println!("Limit reached");
                break;
            }
        }

        // Parameters for the request
        let params = [
            ("symbol", symbol),
            ("interval", &interval.to_binance_string()),
            ("limit", KLINES_LIMIT),
            (time_string, &time_param.to_string()),
        ];

        // Fetch klines
        let klines = fetch_klines(client, &params).await;
        if klines.len() <= 1 {
            println!("No more recent data to fetch");
            break;
        }

        // Update the time parameter
        time_param = match fetch_type {
            KlinesFetchType::Recent => klines.last(),
            KlinesFetchType::Older => klines.first(),
        }
        .unwrap()[0]
            .as_u64()
            .expect("Failed to get open time");

        // Insert the klines
        for kline in klines {
            let result = insert_kline(pool, table_name, &kline).await;
            if result.is_ok() {
                *table_length += 1;
            }
        }
    }
}

async fn fetch_klines(client: &Client, params: &[(&str, &str)]) -> Vec<serde_json::Value> {
    let end_time_human = chrono::Utc
        .timestamp_millis_opt(params[3].1.parse().unwrap())
        .single();
    print!(
        "Fetching klines... Time : {:?}",
        end_time_human.unwrap().to_rfc2822()
    );

    let response = client
        .get(&format!("{}/fapi/v1/klines", BINANCE_FUTURES_API_URL))
        .query(params)
        .send()
        .await
        .expect("Failed to get klines")
        .text()
        .await
        .expect("Failed to convert response to string");

    let data: serde_json::Value = serde_json::from_str(&response).expect("Failed to parse klines");
    sleep(Duration::from_secs(REQUESTS_DELAY_SEC)).await; // Add a delay between requests
    data.as_array().expect("Failed to get klines").clone()
}

async fn insert_kline(
    pool: &PgPool,
    table_name: &str,
    kline: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let kline = kline.as_array().expect("Failed to get kline");
    let open_time = kline[0].as_u64().expect("Failed to get open time");
    let open = utils::json_value_to_f64(&kline[1]);
    let high = utils::json_value_to_f64(&kline[2]);
    let low = utils::json_value_to_f64(&kline[3]);
    let close = utils::json_value_to_f64(&kline[4]);
    let volume = utils::json_value_to_f64(&kline[5]);
    let close_time = kline[6].as_u64().expect("Failed to get close time");
    // If close time is after now, skip the kline
    if close_time > chrono::Utc::now().timestamp_millis() as u64 {
        return Err(sqlx::Error::ColumnNotFound(
            "Close time is after now".to_string(),
        ));
    }
    let quote_asset_volume = utils::json_value_to_f64(&kline[7]);
    let number_of_trades = kline[8].as_u64().expect("Failed to get number of trades");
    let taker_buy_base_asset_volume = utils::json_value_to_f64(&kline[9]);
    let taker_buy_quote_asset_volume = utils::json_value_to_f64(&kline[10]);

    let result = sqlx::query(&format!(
        r#"
        INSERT INTO {} (open_time, open, high, low, close, volume, close_time, quote_asset_volume, number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        table_name
    ))
    .bind(open_time as i64)
    .bind(open)
    .bind(high)
    .bind(low)
    .bind(close)
    .bind(volume)
    .bind(close_time as i64)
    .bind(quote_asset_volume)
    .bind(number_of_trades as i64)
    .bind(taker_buy_base_asset_volume)
    .bind(taker_buy_quote_asset_volume)
    .execute(pool)
    .await;

    if let Err(e) = result {
        // println!("Failed to insert kline: {}", e);
        return Err(e);
    }

    Ok(())
}

async fn check_klines_integrity(pool: &PgPool, table_name: &str) {
    // Check the number of klines in the table
    let result: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COUNT(*) FROM {}
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get klines count");

    print!("Klines count: {} / ", result);

    // Check if each difference between the open time of two consecutive klines is equal to the interval
    // Get interval between two consecutive klines (first kline open time - second kline open time)
    // We will then count the number of differences that are not equal to this interval
    // Get the interval
    let intervals: Vec<Option<i64>> = sqlx::query_scalar(&format!(
        r#"
        SELECT open_time - LAG(open_time, 1) OVER (ORDER BY open_time) AS diff
        FROM {}
        LIMIT 2
        "#,
        table_name
    ))
    .fetch_all(pool)
    .await
    .expect("Failed to get interval");

    let interval: i64 = intervals.last().unwrap().expect("Failed to get interval");

    print!("Interval: {} / ", interval);

    //
    let result: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COUNT(*) FROM (
            SELECT open_time - LAG(open_time, 1) OVER (ORDER BY open_time) AS diff
            FROM {}
        ) AS diffs
        WHERE diff != {}
        "#,
        table_name, interval
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get differences count");

    print!("Differences count: {} / ", result);

    if result == 0 {
        println!("=> Klines integrity is OK");
    } else {
        panic!("=> Klines integrity is NOT OK");
    }
}
