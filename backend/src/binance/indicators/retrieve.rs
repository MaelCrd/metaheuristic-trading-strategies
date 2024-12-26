use super::super::klines;
use super::compute;
use crate::objects::{
    indicators::{Indicator, IndicatorTrait},
    klines::KlineCollection,
};

// Retrieve from the database the klines indicators for the given symbol, interval and limit
// if the klines indicators are not present in the database, they are computed and stored
pub async fn retrieve_indicator(
    kline_collection: &KlineCollection,
    indicator: &mut Indicator,
) -> Result<(), sqlx::Error> {
    // Retrieve the klines indicators from the database
    // We assume klines are already present in the database
    let table_name: String = klines::utils::get_table_name_collection(kline_collection);
    let indicator_columns: Vec<String> = indicator.column_names();
    let pool = klines::utils::connect_to_db().await;

    // Check if the indicator columns exists
    let columns_exists =
        klines::utils::check_columns_exists(&pool, &table_name, &indicator_columns).await;

    // If the indicator columns does not exist, create them
    if !columns_exists {
        println!("Creating indicator columns...");
        klines::utils::create_indicator_columns(&pool, &table_name, &indicator_columns)
            .await
            .unwrap();
    }

    // Query the database to retrieve the open_time of rows that are missing the indicator columns
    // and between the first and last open time
    let first_open_time = kline_collection.get_first_open_time();
    let last_open_time = kline_collection.get_last_open_time();
    let result = sqlx::query(&format!(
        r#"
        SELECT open_time
        FROM {}
        WHERE ({} IS NULL)
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

    // println!("First open time: {:?}", first_open_time);
    // println!("Last open time: {:?}", last_open_time);

    let missing_rows = result.len();

    println!("Missing rows: {}", missing_rows); //, result);

    // Compute the indicator columns for the missing rows
    if missing_rows > 0 {
        compute::compute_indicator(&pool, indicator, kline_collection)
            .await
            .unwrap();
    }

    // Query the database to retrieve the indicator columns
    let result = sqlx::query(&format!(
        r#"
        SELECT {}
        FROM {}
        WHERE open_time BETWEEN {} AND {}
        ORDER BY open_time ASC
        "#,
        indicator.column_names().join(", "),
        table_name,
        first_open_time.timestamp_millis(),
        last_open_time.timestamp_millis()
    ))
    .fetch_all(&pool)
    .await
    .unwrap();

    // Store the indicator columns in the kline collection
    indicator.store_rows(&result);

    Ok(())
}

pub async fn retrieve_extended_klines(
    kline_collection: &mut KlineCollection,
    indicator: &Indicator,
) -> Result<(), sqlx::Error> {
    // Retrieve the number of rows needed for the indicator to be computed on the first row
    let n_before_needed = indicator.n_before_needed();

    // Retrieve the klines needed to compute the indicator (extended collection)
    klines::retrieve::retrieve_klines_extend(kline_collection, n_before_needed)
        .await
        .unwrap();

    Ok(())
}

pub async fn retrieve_extended_klines_max(
    kline_collection: &mut KlineCollection,
    indicators: &Vec<Indicator>,
) -> Result<(), sqlx::Error> {
    // Retrieve the number of rows needed for the indicator to be computed on the first row
    let max_indicator = indicators
        .iter()
        .max_by_key(|x| x.n_before_needed())
        .unwrap();

    // Retrieve the klines needed to compute the indicator (extended collection)
    retrieve_extended_klines(kline_collection, &max_indicator)
        .await
        .unwrap();

    Ok(())
}
