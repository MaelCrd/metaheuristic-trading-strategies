//! File with all the objects that are used in the database.

// Import the necessary modules
use serde::{Deserialize, Serialize};

// --- Crypto Objects --- //

#[derive(Deserialize)]
pub struct CreateCryptoObject {
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize)]
pub struct CryptoObject {
    pub id: i32,
    pub name: String,
    pub r#type: String,
}

// --- Other Objects --- //

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub version: String,
}
