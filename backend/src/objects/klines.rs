// Import the necessary modules
use chrono::{DateTime, Utc}; // NaiveDateTime
use serde::{Deserialize, Serialize};

use super::intervals;
use crate::binance::klines;

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

impl Kline {
    pub fn display(&self) {
        println!(
            "{{Open time: {:?}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}, Close time: {:?}, Quote asset volume: {}, Number of trades: {}, Taker buy base asset volume: {}, Taker buy quote asset volume: {}}}",
            self.open_time,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
            self.close_time,
            self.quote_asset_volume,
            self.number_of_trades,
            self.taker_buy_base_asset_volume,
            self.taker_buy_quote_asset_volume
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineCollection {
    pub symbol: String,
    pub interval: intervals::CryptoInterval,
    pub training_percentage: f64,
    pub training: Vec<Kline>,
    pub validation: Vec<Kline>,
    pub past: Vec<Kline>,
}

impl KlineCollection {
    pub fn new() -> KlineCollection {
        KlineCollection {
            symbol: String::new(),
            interval: intervals::CryptoInterval::Int1m,
            training_percentage: 1.0,
            training: Vec::new(),
            validation: Vec::new(),
            past: Vec::new(),
        }
    }

    // Return the kline at the given index (normal)
    // (0 -> first training, 1 -> second training, ... , last validation)
    pub fn get(&self, index: i32) -> Option<&Kline> {
        let validation_len = self.validation.len() as i32;
        let training_len = self.training.len() as i32;
        let past_len = self.past.len() as i32;
        let total = validation_len + training_len + past_len;
        if index < 0 || index >= total {
            return None;
        }
        if index < training_len {
            return self.training.get(index as usize);
        }
        if index < training_len + validation_len {
            return self.validation.get((index - training_len) as usize);
        }
        self.past
            .get((index - training_len - validation_len) as usize)
    }

    // Return the kline at the given index (reverse)
    // (0 -> last validation, 1 -> one before last validation, ... , first past)
    pub fn get_rev(&self, index: i32) -> Option<&Kline> {
        let validation_len = self.validation.len() as i32;
        let training_len = self.training.len() as i32;
        let past_len = self.past.len() as i32;
        let total = validation_len + training_len + past_len;
        if index < 0 || index >= total {
            return None;
        }
        if index < validation_len {
            return self.validation.get((validation_len - 1 - index) as usize);
        }
        if index < validation_len + training_len {
            return self
                .training
                .get((validation_len + training_len - 1 - index) as usize);
        }
        self.past
            .get((validation_len + training_len + past_len - 1 - index) as usize)
    }

    pub fn get_length(&self) -> i32 {
        self.training.len() as i32 + self.validation.len() as i32
    }

    pub fn get_limit_minutes(&self) -> i64 {
        self.get_length() as i64 * self.interval.to_minutes()
    }

    pub fn get_last_open_time(&self) -> DateTime<Utc> {
        if self.validation.len() > 0 {
            self.validation.last().unwrap().open_time
        } else if self.training.len() > 0 {
            self.training.last().unwrap().open_time
        } else {
            Utc::now()
        }
    }

    pub fn get_first_open_time(&self) -> DateTime<Utc> {
        if self.training.len() > 0 {
            self.training.first().unwrap().open_time
        } else if self.validation.len() > 0 {
            self.validation.first().unwrap().open_time
        } else {
            Utc::now()
        }
    }

    pub fn get_first_past_open_time(&self) -> DateTime<Utc> {
        if self.past.len() > 0 {
            self.past.first().unwrap().open_time
        } else {
            self.get_first_open_time()
        }
    }

    pub fn display(&self) {
        println!(
            "{{Symbol: {}, Interval: {}, Training percentage: {}, Lengths: {},{},({}), Limit minutes: {}}}",
            self.symbol,
            self.interval.to_string(),
            self.training_percentage,
            self.training.len(),
            self.validation.len(),
            self.past.len(),
            self.get_limit_minutes()
        );
    }

    pub fn check_integrity(&self) {
        if klines::utils::check_klines_collection_integrity(self) {
            println!("Klines collection integrity passed ✓");
        } else {
            println!("Klines collection integrity FAILED X");
        }
    }
}
