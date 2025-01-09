use rocket::get;
use rocket::serde::json::Json;

use crate::objects::indicators::{self, IndicatorInformation};

// Define a route to get all Indicators
#[get("/indicators")]
pub async fn get_indicators() -> Json<Vec<IndicatorInformation>> {
    Json(indicators::Indicator::get_all_indicators_info())
}
