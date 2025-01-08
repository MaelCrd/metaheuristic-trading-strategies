use std::time::{Duration, Instant};

// Structure to hold thread status information
#[derive(Debug, Clone)]
pub struct ThreadStatus {
    pub is_complete: bool,
    pub success: bool,
    pub start_time: Instant,
    pub duration: Duration,
    pub result: String,
}
