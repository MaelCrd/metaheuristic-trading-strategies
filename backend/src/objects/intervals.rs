// Import the necessary modules
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "crypto_interval")]
pub enum CryptoInterval {
    Int1m,
    Int5m,
    Int15m,
    Int30m,
    Int1h,
    Int2h,
    Int4h,
    Int6h,
    Int8h,
    Int12h,
    Int1d,
    Int3d,
    Int1w,
    Int1M,
}

impl CryptoInterval {
    // Parse from string
    pub fn parse_from(interval: &str) -> CryptoInterval {
        match interval {
            "Int1m" => CryptoInterval::Int1m,
            "Int5m" => CryptoInterval::Int5m,
            "Int15m" => CryptoInterval::Int15m,
            "Int30m" => CryptoInterval::Int30m,
            "Int1h" => CryptoInterval::Int1h,
            "Int2h" => CryptoInterval::Int2h,
            "Int4h" => CryptoInterval::Int4h,
            "Int6h" => CryptoInterval::Int6h,
            "Int8h" => CryptoInterval::Int8h,
            "Int12h" => CryptoInterval::Int12h,
            "Int1d" => CryptoInterval::Int1d,
            "Int3d" => CryptoInterval::Int3d,
            "Int1w" => CryptoInterval::Int1w,
            "Int1M" => CryptoInterval::Int1M,
            _ => CryptoInterval::Int1m,
        }
    }

    // To string
    pub fn to_string(&self) -> String {
        match self {
            CryptoInterval::Int1m => "Int1m".to_string(),
            CryptoInterval::Int5m => "Int5m".to_string(),
            CryptoInterval::Int15m => "Int15m".to_string(),
            CryptoInterval::Int30m => "Int30m".to_string(),
            CryptoInterval::Int1h => "Int1h".to_string(),
            CryptoInterval::Int2h => "Int2h".to_string(),
            CryptoInterval::Int4h => "Int4h".to_string(),
            CryptoInterval::Int6h => "Int6h".to_string(),
            CryptoInterval::Int8h => "Int8h".to_string(),
            CryptoInterval::Int12h => "Int12h".to_string(),
            CryptoInterval::Int1d => "Int1d".to_string(),
            CryptoInterval::Int3d => "Int3d".to_string(),
            CryptoInterval::Int1w => "Int1w".to_string(),
            CryptoInterval::Int1M => "Int1M".to_string(),
        }
    }

    // To Binance string
    pub fn to_binance_string(&self) -> String {
        match self {
            CryptoInterval::Int1m => "1m".to_string(),
            CryptoInterval::Int5m => "5m".to_string(),
            CryptoInterval::Int15m => "15m".to_string(),
            CryptoInterval::Int30m => "30m".to_string(),
            CryptoInterval::Int1h => "1h".to_string(),
            CryptoInterval::Int2h => "2h".to_string(),
            CryptoInterval::Int4h => "4h".to_string(),
            CryptoInterval::Int6h => "6h".to_string(),
            CryptoInterval::Int8h => "8h".to_string(),
            CryptoInterval::Int12h => "12h".to_string(),
            CryptoInterval::Int1d => "1d".to_string(),
            CryptoInterval::Int3d => "3d".to_string(),
            CryptoInterval::Int1w => "1w".to_string(),
            CryptoInterval::Int1M => "1M".to_string(),
        }
    }

    // To minutes
    pub fn to_minutes(&self) -> i64 {
        match self {
            CryptoInterval::Int1m => 1,
            CryptoInterval::Int5m => 5,
            CryptoInterval::Int15m => 15,
            CryptoInterval::Int30m => 30,
            CryptoInterval::Int1h => 60,
            CryptoInterval::Int2h => 120,
            CryptoInterval::Int4h => 240,
            CryptoInterval::Int6h => 360,
            CryptoInterval::Int8h => 480,
            CryptoInterval::Int12h => 720,
            CryptoInterval::Int1d => 1440,
            CryptoInterval::Int3d => 4320,
            CryptoInterval::Int1w => 10080,
            CryptoInterval::Int1M => 43200,
        }
    }
}
