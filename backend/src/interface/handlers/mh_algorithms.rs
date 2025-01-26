use rocket::get;
use rocket::serde::json::Json;

use crate::metaheuristic;

// Define a route to get all algorithms
#[get("/algorithms")]
pub async fn get_algorithms() -> Json<Vec<metaheuristic::MetaheuristicInfo>> {
    Json(metaheuristic::Metaheuristic::get_all_info())
}

// // Define a route to get indicators information for a specific indicator combination
// #[get("/indicators_in_combination?<id>")]
// pub async fn get_indicators_in_combination(
//     pool: &State<PgPool>,
//     id: String,
// ) -> Json<Vec<indicators::IndicatorInCombination>> {
//     // Get all the indicators in the indicator combination
//     let indicators_in_combination = sqlx::query(
//         format!(
//             r#"
//             SELECT indicator_combination_id, indicator_struct_name, parameters
//             FROM indicator_in_combination
//             WHERE indicator_combination_id = {}
//             "#,
//             id
//         )
//         .as_str(),
//     )
//     .fetch_all(&**pool)
//     .await
//     .unwrap();

//     // Convert the records to IndicatorInformation objects
//     let indicators_in_combination: Vec<indicators::IndicatorInCombination> =
//         futures::future::join_all(indicators_in_combination.into_iter().map(|row| async move {
//             indicators::IndicatorInCombination {
//                 indicator_combination_id: row.get("indicator_combination_id"),
//                 indicator_struct_name: row.get("indicator_struct_name"),
//                 parameters: row.get("parameters"),
//             }
//         }))
//         .await;

//     Json(indicators_in_combination)
// }

// // Define a route to get all indicator combinations
// #[get("/indicator_combinations?<id>")]
// pub async fn get_indicator_combinations(
//     pool: &State<PgPool>,
//     id: Option<String>,
// ) -> Json<Vec<indicators::CompleteIndicatorCombination>> {
//     // If an id is provided, get the indicator combination with that id
//     let request_addition = if let Some(id) = id {
//         format!("WHERE id = {}", id)
//     } else {
//         "".to_string()
//     };

//     // Get all the indicator combinations
//     let indicator_combinations = sqlx::query(
//         format!(
//             r#"
//         SELECT id, name, hidden
//         FROM indicator_combination {}
//         "#,
//             request_addition
//         )
//         .as_str(),
//     )
//     .fetch_all(&**pool)
//     .await
//     .unwrap();

//     // Convert the records to IndicatorCombination objects
//     let indicator_combinations: Vec<indicators::CompleteIndicatorCombination> =
//         futures::future::join_all(indicator_combinations.into_iter().map(|row| async move {
//             let indicators_struct_names = sqlx::query!(
//                 r#"
//                 SELECT indicator_struct_name
//                 FROM indicator_in_combination
//                 WHERE indicator_combination_id = $1
//                 "#,
//                 row.get::<i32, _>("id"),
//             )
//             .fetch_all(&**pool)
//             .await
//             .unwrap()
//             .into_iter()
//             .map(|row| row.indicator_struct_name)
//             .collect();

//             indicators::CompleteIndicatorCombination {
//                 id: row.get("id"),
//                 name: row.get("name"),
//                 indicators_struct_names,
//                 hidden: row.get("hidden"),
//             }
//         }))
//         .await;

//     Json(indicator_combinations)
// }

// // Define a route to create an indicator combination
// #[post("/indicator_combinations", data = "<create_indicator_combination>")]
// pub async fn create_indicator_combination(
//     pool: &State<PgPool>,
//     create_indicator_combination: Json<indicators::CreateIndicatorCombination>,
// ) -> Result<Json<Vec<indicators::CompleteIndicatorCombination>>, rocket::http::Status> {
//     // Send the request
//     let res = sqlx::query!(
//         r#"
//         INSERT INTO indicator_combination (name, hidden)
//         VALUES ($1, $2)
//         RETURNING id
//         "#,
//         create_indicator_combination.name,
//         false,
//     )
//     .fetch_one(&**pool)
//     .await
//     .unwrap();

//     // Get the id of the new indicator combination
//     let indicator_combination_id = res.id;

//     // Link the indicators to the new indicator combination using the indicator_combination_x_indicator table
//     for indicator in create_indicator_combination.indicators.iter() {
//         println!("Indicator: {}", indicator);
//         let indicator_info =
//             serde_json::from_str::<CreateIndicatorInCombination>(indicator.as_str()).unwrap();
//         let result = sqlx::query!(
//             r#"
//             INSERT INTO indicator_in_combination (indicator_combination_id, indicator_struct_name, parameters)
//             VALUES ($1, $2, $3)
//             "#,
//             indicator_combination_id,
//             indicator_info.indicator_struct_name,
//             indicator_info.parameters,
//         )
//         .execute(&**pool)
//         .await;

//         if let Err(e) = result {
//             println!(
//                 "Failed to link indicator to indicator combination: {}\nReverting changes...",
//                 e
//             );

//             // Delete the indicators linked to the indicator combination
//             let _ = sqlx::query!(
//                 r#"
//                 DELETE FROM indicator_in_combination
//                 WHERE indicator_combination_id = $1
//                 "#,
//                 indicator_combination_id,
//             )
//             .execute(&**pool)
//             .await
//             .unwrap();

//             // Delete the indicator combination
//             let _ = sqlx::query!(
//                 r#"
//                 DELETE FROM indicator_combination
//                 WHERE id = $1
//                 "#,
//                 indicator_combination_id,
//             )
//             .execute(&**pool)
//             .await
//             .unwrap();

//             // Return an internal server error
//             return Err(rocket::http::Status::InternalServerError);
//         }
//     }

//     Ok(get_indicator_combinations(pool, None).await)
// }
