use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use sqlx::types::time::PrimitiveDateTime;

pub fn convert_primitive_to_chrono(primitive: PrimitiveDateTime) -> DateTime<Utc> {
    let date_time_str = primitive.to_string();
    let naive_datetime = NaiveDateTime::parse_from_str(&date_time_str, "%Y-%m-%d %H:%M:%S%.f")
        .expect("Failed to parse datetime");
    Utc.from_utc_datetime(&naive_datetime).into()
}
