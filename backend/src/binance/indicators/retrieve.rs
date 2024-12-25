use sqlx::Row;

use super::super::klines::utils;
use super::compute;
use crate::objects::{indicators::Indicator, indicators::IndicatorTrait, objects::KlineCollection};

// Retrieve from the database the klines indicators for the given symbol, interval and limit
// if the klines indicators are not present in the database, they are computed and stored
pub async fn retrieve_indicator(
    kline_collection: &KlineCollection,
    indicator: &mut Indicator,
) -> Result<(), sqlx::Error> {
    // Retrieve the klines indicators from the database
    // We assume klines are already present in the database
    let table_name = utils::get_table_name_collection(kline_collection);
    let indicator_columns: Vec<String> = indicator.column_names();
    let pool = utils::connect_to_db().await;

    // Check if the indicator columns exists
    let columns_exists = utils::check_columns_exists(&pool, &table_name, &indicator_columns).await;

    // If the indicator columns does not exist, create them
    if !columns_exists {
        println!("Creating indicator columns...");
        utils::create_indicator_columns(&pool, &table_name, &indicator_columns)
            .await
            .unwrap();
    }

    // Compute the indicator columns
    compute::compute_indicator(&pool, indicator, kline_collection)
        .await
        .unwrap();

    // // Query the database to retrieve the klines indicators
    // let result = sqlx::query(&format!(
    //     r#"
    //     SELECT open_time, {}
    //     FROM {}
    //     ORDER BY open_time DESC
    //     LIMIT {}
    //     "#,
    //     indicator.column_names().join(", "),
    //     table_name,
    //     kline_collection.get_length()
    // ))
    // .fetch_all(&pool)
    // .await
    // .unwrap();

    // let result = result.iter().map(|row| row.get(0)).collect::<Vec<i64>>();

    // println!("Rows without indicators: {:?}", result.len());

    // // For each row without indicators, compute the indicators
    // if result.len() > 0 {
    //     compute::compute_indicator(&pool, indicator, kline_collection).await.unwrap();
    // }

    // Query the database to retrieve the klines indicators

    Ok(())
}
