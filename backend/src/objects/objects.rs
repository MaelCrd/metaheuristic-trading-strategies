//! File with all the objects that are used in the database.

// Import the necessary modules
use chrono::{DateTime, Utc}; // NaiveDateTime
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoList {
    pub id: i32,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCryptoList {
    pub name: String,
    pub r#type: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub state: String,
    pub other_parameters: Option<String>,
    pub mh_object_id: Option<i32>,
    pub crypto_list_id: Option<i32>,
    pub result_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub state: String,
    pub other_parameters: Option<String>,
    pub mh_object_id: Option<i32>,
    pub crypto_list_id: Option<i32>,
    pub result_id: Option<i32>,
}

// --- Other Objects --- //

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub version: String,
}
