use crate::objects::objects::CryptoInterval;

pub fn get_table_name(symbol: &str, interval: &CryptoInterval) -> String {
    format!("klines_{}_{}", symbol, interval.to_string()).to_lowercase()
}
