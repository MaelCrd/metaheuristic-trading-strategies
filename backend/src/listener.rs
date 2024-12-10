use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReceivedData {
    received_message: String,
    received_value: i32,
    processed: bool,
}

// Method 1: Using a channel to receive updates
pub async fn listen_for_updates(mut rx: mpsc::Receiver<i32>) {
    while let Some(data) = rx.recv().await {
        println!("Received update: {:?}", data);
        // Process the data here
    }
}
