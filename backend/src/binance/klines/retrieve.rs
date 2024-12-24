use chrono::DateTime;
use chrono::Utc;
use sqlx::PgPool;
use sqlx::Row;

use super::utils;
use crate::objects::objects::CryptoInterval;
use crate::objects::objects::Kline;

// Function to retrieve klines from database
pub async fn retrieve_klines(
    pool: &PgPool,
    klines: &mut Vec<Kline>,
    symbol: &str,
    interval: &CryptoInterval,
    limit_minutes: i64,
) -> Result<(), sqlx::Error> {
    // Retrieve the klines from the database
    let table_name = utils::get_table_name(symbol, interval);
    let limit = limit_minutes / interval.to_minutes();
    let result = sqlx::query(&format!(
        r#"
        SELECT open_time, open, high, low, close, volume, close_time, quote_asset_volume, number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume
        FROM {}
        ORDER BY open_time DESC
        LIMIT {}
        "#,
        table_name, limit
    )).fetch_all(pool).await.unwrap();

    // Parse the results
    for row in result {
        klines.push(Kline {
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
    }

    Ok(())
}
