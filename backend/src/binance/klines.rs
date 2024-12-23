use chrono::TimeZone;
use reqwest::Client;
use sqlx::PgPool;
use std::{env, time::Duration};
use tokio::time::sleep;

const BINANCE_FUTURES_API_URL: &str = "https://fapi.binance.com";
const KLINES_LIMIT: &str = "1500";

pub async fn get_klines(symbol: &str, interval: &str) {
    let pool = connect_to_db().await;
    let table_name = format!("klines_{}_{}", symbol, interval).to_lowercase();
    println!("Table name: {}", table_name);

    let table_exists = check_table_exists(&pool, &table_name).await;
    let mut table_empty = true;

    if table_exists {
        table_empty = check_table_empy(&pool, &table_name).await;
    }

    println!("Table empty: {}", table_empty);

    let client = Client::new();

    if !table_exists || table_empty {
        println!("Table doesn't exist, creating table and fetching historical data...");
        if !table_exists {
            create_table(&pool, &table_name).await;
        }
        fetch_historical_data(&pool, &client, &table_name, symbol, interval).await;
    } else {
        println!("Table exists, fetching recent and older data...");
        fetch_recent_data(&pool, &client, &table_name, symbol, interval).await;
        fetch_older_data(&pool, &client, &table_name, symbol, interval).await;
    }

    // Check the integrity of the klines
    check_klines_integrity(&pool, &table_name).await;
}

async fn connect_to_db() -> PgPool {
    PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool.")
}

async fn check_table_exists(pool: &PgPool, table_name: &str) -> bool {
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

async fn check_table_empy(pool: &PgPool, table_name: &str) -> bool {
    let result: bool = sqlx::query_scalar(&format!(
        r#"
        SELECT EXISTS (
            SELECT FROM {}
        )
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to check if table is empty");

    !result
}

async fn create_table(pool: &PgPool, table_name: &str) {
    sqlx::query(&format!(
        r#"
        CREATE TABLE {} (
            open_time BIGINT PRIMARY KEY,
            open TEXT NOT NULL,
            high TEXT NOT NULL,
            low TEXT NOT NULL,
            close TEXT NOT NULL,
            volume TEXT NOT NULL,
            close_time BIGINT NOT NULL,
            quote_asset_volume TEXT NOT NULL,
            number_of_trades BIGINT NOT NULL,
            taker_buy_base_asset_volume TEXT NOT NULL,
            taker_buy_quote_asset_volume TEXT NOT NULL
        )
        "#,
        table_name
    ))
    .execute(pool)
    .await
    .expect("Failed to create table");
}

async fn fetch_historical_data(
    pool: &PgPool,
    client: &Client,
    table_name: &str,
    symbol: &str,
    interval: &str,
) {
    println!("Fetching historical data...");
    // Set end time to timestamp of 1 year after the current time
    let mut end_time: u64 = chrono::Utc::now().timestamp_millis() as u64 + 31536000000;
    loop {
        let params = {
            [
                ("symbol", symbol),
                ("interval", interval),
                ("limit", KLINES_LIMIT),
                ("endTime", &end_time.to_string()),
            ]
        };

        let klines = fetch_klines(client, &params).await;
        if klines.len() <= 1 {
            println!("No more historical data to fetch");
            break;
        }

        end_time = klines.first().unwrap()[0]
            .as_u64()
            .expect("Failed to get open time");
        for kline in klines {
            insert_kline(pool, table_name, &kline).await;
        }
    }
}

async fn fetch_older_data(
    pool: &PgPool,
    client: &Client,
    table_name: &str,
    symbol: &str,
    interval: &str,
) {
    println!("Fetching older data...");
    let result: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT MIN(open_time) FROM {}
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get minimum open_time");

    let mut end_time = result.abs() as u64;

    loop {
        let params = [
            ("symbol", symbol),
            ("interval", interval),
            ("limit", KLINES_LIMIT),
            ("endTime", &end_time.to_string()),
        ];

        let klines = fetch_klines(client, &params).await;
        if klines.len() <= 1 {
            println!("No more older data to fetch");
            break;
        }

        end_time = klines.first().unwrap()[0]
            .as_u64()
            .expect("Failed to get open time");
        for kline in klines {
            insert_kline(pool, table_name, &kline).await;
        }
    }
}

async fn fetch_recent_data(
    pool: &PgPool,
    client: &Client,
    table_name: &str,
    symbol: &str,
    interval: &str,
) {
    println!("Fetching recent data...");
    let result: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT MAX(open_time) FROM {}
        "#,
        table_name
    ))
    .fetch_one(pool)
    .await
    .expect("Failed to get maximum open_time");

    let mut start_time = result.abs() as u64;

    loop {
        let params = [
            ("symbol", symbol),
            ("interval", interval),
            ("limit", KLINES_LIMIT),
            ("startTime", &start_time.to_string()),
        ];

        let klines = fetch_klines(client, &params).await;
        if klines.len() <= 1 {
            println!("No more recent data to fetch");
            break;
        }

        start_time = klines.last().unwrap()[0]
            .as_u64()
            .expect("Failed to get open time");
        for kline in klines {
            insert_kline(pool, table_name, &kline).await;
        }
    }
}

async fn fetch_klines(client: &Client, params: &[(&str, &str)]) -> Vec<serde_json::Value> {
    let end_time_human = chrono::Utc
        .timestamp_millis_opt(params[3].1.parse().unwrap())
        .single();
    println!(
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
    // sleep(Duration::from_secs(3)).await; // Add a 1-second delay between requests
    data.as_array().expect("Failed to get klines").clone()
}

async fn insert_kline(pool: &PgPool, table_name: &str, kline: &serde_json::Value) {
    let kline = kline.as_array().expect("Failed to get kline");
    let open_time = kline[0].as_u64().expect("Failed to get open time");
    let open = kline[1].as_str().expect("Failed to get open");
    let high = kline[2].as_str().expect("Failed to get high");
    let low = kline[3].as_str().expect("Failed to get low");
    let close = kline[4].as_str().expect("Failed to get close");
    let volume = kline[5].as_str().expect("Failed to get volume");
    let close_time = kline[6].as_u64().expect("Failed to get close time");
    let quote_asset_volume = kline[7].as_str().expect("Failed to get quote asset volume");
    let number_of_trades = kline[8].as_u64().expect("Failed to get number of trades");
    let taker_buy_base_asset_volume = kline[9]
        .as_str()
        .expect("Failed to get taker buy base asset volume");
    let taker_buy_quote_asset_volume = kline[10]
        .as_str()
        .expect("Failed to get taker buy quote asset volume");

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

    // match result {
    //     Ok(_) => println!("Inserted kline with open time: {}", open_time),
    //     Err(e) => println!("Failed to insert kline: {}", e),
    // }

    if let Err(e) = result {
        println!("Failed to insert kline: {}", e);
    }
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

    println!("Klines count: {}", result);

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

    println!("Interval: {}", interval);

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

    println!("Differences count: {}", result);

    if result == 0 {
        println!("Klines integrity is OK");
    } else {
        panic!("Klines integrity is NOT OK");
    }
}
