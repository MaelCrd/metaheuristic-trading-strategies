//! File with all the objects that are used in the database.

// Import the necessary modules
use chrono::{DateTime, Utc}; // NaiveDateTime
use serde::{Deserialize, Serialize};
use sqlx::Type;

use super::intervals;

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

impl CryptoSymbol {
    pub fn new() -> CryptoSymbol {
        CryptoSymbol {
            id: -1,
            symbol: "".to_string(),
            name: "".to_string(),
            volume: -1.0,
            last_updated: Utc::now(),
            available: false,
        }
    }
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
    pub hidden: bool,
    pub name: String,
    pub interval: intervals::CryptoInterval,
    pub limit_count: i32,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoListComplete {
    pub id: i32,
    pub hidden: bool,
    pub name: String,
    pub interval: intervals::CryptoInterval,
    pub limit_count: i32,
    pub r#type: String,
    pub crypto_symbols: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCryptoList {
    pub name: String,
    pub interval: intervals::CryptoInterval,
    pub limit_count: i32,
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
    pub mh_algorithm_name: String,
    pub mh_parameters: String,
    pub other_parameters: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMHObject {
    pub mh_algorithm_name: String,
    pub mh_parameters: String,
    pub other_parameters: Option<String>,
}

// --- MH Algorithms --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MHAlgorithm {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub hidden: bool,
    pub parameters: String,
}

// --- Results --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    pub id: i32,
    pub results: String,
    pub other_parameters: Option<String>,
}

// --- Tasks --- //

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
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
    pub indicator_combination_id: Option<i32>,
    pub result_id: Option<i32>,
}

impl Task {
    pub fn new(id: i32, state: TaskState, created_at: DateTime<Utc>) -> Task {
        Task {
            id,
            state,
            created_at,
            other_parameters: None,
            mh_object_id: None,
            crypto_list_id: None,
            indicator_combination_id: None,
            result_id: None,
        }
    }

    pub async fn set_state(&mut self, pool: &sqlx::PgPool, state: TaskState) {
        match crate::interface::handlers::tasks::update_task_state(
            &rocket::State::from(pool),
            self.id,
            state.clone(),
        )
        .await
        {
            Ok(_) => {
                self.state = state;
            }
            Err(_) => {
                println!("Error updating task state");
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub other_parameters: Option<String>,
    pub mh_object_id: i32,
    pub crypto_list_id: i32,
    pub indicator_combination_id: i32,
}

// --- Other Objects --- //

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub version: String,
}
