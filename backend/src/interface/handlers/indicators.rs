use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};
use sqlx::PgPool;

use crate::objects::indicators::{self, IndicatorInformation};

// Define a route to get all Indicators
#[get("/indicators")]
pub async fn get_indicators() -> Json<Vec<IndicatorInformation>> {
    Json(indicators::Indicator::get_all_indicators_info())
}

// Define a route to get all indicator combinations
#[get("/indicator_combinations")]
pub async fn get_indicator_combinations(
    pool: &State<PgPool>,
) -> Json<Vec<indicators::CompleteIndicatorCombination>> {
    // Get all the indicator combinations
    let indicator_combinations = sqlx::query!(
        r#"
        SELECT id, name, hidden
        FROM indicator_combination
        "#,
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    // Convert the records to IndicatorCombination objects
    let indicator_combinations: Vec<indicators::CompleteIndicatorCombination> =
        futures::future::join_all(indicator_combinations.into_iter().map(|row| async move {
            let indicators_struct_names = sqlx::query!(
                r#"
                SELECT indicator_struct_name
                FROM indicator_in_combination
                WHERE indicator_combination_id = $1
                "#,
                row.id,
            )
            .fetch_all(&**pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.indicator_struct_name)
            .collect();

            indicators::CompleteIndicatorCombination {
                id: row.id,
                name: row.name,
                indicators_struct_names,
                hidden: row.hidden,
            }
        }))
        .await;

    Json(indicator_combinations)
}

// Define a route to create an indicator combination
#[post("/indicator_combinations", data = "<create_indicator_combination>")]
pub async fn create_indicator_combination(
    pool: &State<PgPool>,
    create_indicator_combination: Json<indicators::CreateIndicatorCombination>,
) -> Json<Vec<indicators::CompleteIndicatorCombination>> {
    // Send the request
    let res = sqlx::query!(
        r#"
        INSERT INTO indicator_combination (name, hidden)
        VALUES ($1, $2)
        RETURNING id
        "#,
        create_indicator_combination.name,
        false,
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    // Get the id of the new indicator combination
    let indicator_combination_id = res.id;

    // Link the indicators to the new indicator combination using the indicator_combination_x_indicator table
    for indicator_struct_name in create_indicator_combination.indicators_struct_names.iter() {
        let result = sqlx::query!(
            r#"
            INSERT INTO indicator_in_combination (indicator_combination_id, indicator_struct_name)
            VALUES ($1, $2)
            "#,
            indicator_combination_id,
            indicator_struct_name,
        )
        .execute(&**pool)
        .await;

        if let Err(e) = result {
            println!(
                "Failed to link indicator to indicator combination: {}\nReverting changes...",
                e
            );

            // Delete the indicators linked to the indicator combination
            let _ = sqlx::query!(
                r#"
                DELETE FROM indicator_in_combination
                WHERE indicator_combination_id = $1
                "#,
                indicator_combination_id,
            )
            .execute(&**pool)
            .await
            .unwrap();

            // Delete the indicator combination
            let _ = sqlx::query!(
                r#"
                DELETE FROM indicator_combination
                WHERE id = $1
                "#,
                indicator_combination_id,
            )
            .execute(&**pool)
            .await
            .unwrap();

            // Return an internal server error
            return get_indicator_combinations(pool).await;
        }
    }

    get_indicator_combinations(pool).await
}
