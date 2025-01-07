use super::super::klines::utils;
use crate::objects::indicators::IndicatorTrait;
use crate::objects::{indicators::Indicator, klines::KlineCollection};

pub async fn compute_indicator(
    indicator: &mut Indicator,
    kline_collection: &KlineCollection,
) -> Result<(), sqlx::Error> {
    // We assume klines are already present in the database
    let table_name = utils::get_table_name_collection(&kline_collection);

    // Check if the table exists
    let pool = crate::utils::db::get_new_pool().await;

    // Calculate indicator values
    indicator.calculate(kline_collection);

    //
    println!("Indicator values: {:?}", indicator);

    // Insert the indicator values into the database
    let values: Vec<&Vec<Option<f64>>> = indicator.get_values();
    let values_elements_len = values[0].len();
    let columns = indicator.column_names();
    for i in 0..values_elements_len {
        let mut columns_str = String::new();
        for j in 0..values.len() {
            columns_str.push_str(&columns[j]);
            columns_str.push_str(" = ");
            columns_str.push_str(&values[j][i].unwrap().to_string());
            if j < values.len() - 1 {
                columns_str.push_str(", ");
            }
        }

        sqlx::query(&format!(
            r#"
            UPDATE {}
            SET {}
            WHERE open_time = {}
            "#,
            table_name,
            columns_str,
            kline_collection
                .get(i.try_into().unwrap())
                .unwrap()
                .open_time
                .timestamp_millis()
        ))
        .execute(&pool)
        .await
        .unwrap();
    }

    Ok(())
}
