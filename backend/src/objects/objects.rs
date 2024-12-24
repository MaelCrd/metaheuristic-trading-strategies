//! File with all the objects that are used in the database.

// Import the necessary modules
use chrono::{DateTime, Utc}; // NaiveDateTime
use serde::{Deserialize, Serialize};
use sqlx::Type;

// --- Crypto Symbols --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoSymbol {
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub volume: f64,
    pub last_updated: DateTime<Utc>,
    pub available: bool,
}

// Simple version of the CryptoSymbol object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoSymbolSimple {
    pub symbol: String,
    pub volume: f64,
    pub available: bool,
}

// --- Crypto Lists --- //

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoList {
    pub id: i32,
    pub hidden: bool,
    pub name: String,
    pub interval: CryptoInterval,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoListComplete {
    pub id: i32,
    pub hidden: bool,
    pub name: String,
    pub interval: CryptoInterval,
    pub r#type: String,
    pub crypto_symbols: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCryptoList {
    pub name: String,
    pub interval: CryptoInterval,
    pub r#type: String,
    pub crypto_symbols: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoListXCryptoSymbol {
    pub crypto_list_id: i32,
    pub crypto_symbol_id: i32,
}

// --- MH Objects --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MHObject {
    pub id: i32,
    pub hidden: bool,
    pub mh_parameters: String,
    pub other_parameters: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMHObject {
    pub mh_parameters: String,
    pub other_parameters: Option<String>,
}

// --- Results --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    pub id: i32,
    pub results: String,
    pub other_parameters: Option<String>,
}

// --- Tasks --- //

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "state_enum", rename_all = "UPPERCASE")]
pub enum TaskState {
    Created,
    Pending,
    Running,
    Cancelling,
    Cancelled,
    Completed,
    Failed,
}

impl TaskState {
    // Parse from string
    pub fn parse_from(state: &str) -> TaskState {
        match state {
            "CREATED" => TaskState::Created,
            "PENDING" => TaskState::Pending,
            "RUNNING" => TaskState::Running,
            "CANCELLING" => TaskState::Cancelling,
            "CANCELLED" => TaskState::Cancelled,
            "COMPLETED" => TaskState::Completed,
            "FAILED" => TaskState::Failed,
            _ => TaskState::Created,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub state: TaskState,
    pub created_at: DateTime<Utc>,
    pub other_parameters: Option<String>,
    pub mh_object_id: Option<i32>,
    pub crypto_list_id: Option<i32>,
    pub result_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub other_parameters: Option<String>,
    pub mh_object_id: i32,
    pub crypto_list_id: i32,
}

// --- Klines --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    pub open_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: DateTime<Utc>,
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

// --- Other Objects --- //

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub version: String,
}
