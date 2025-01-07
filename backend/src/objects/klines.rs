// Import the necessary modules
use chrono::{DateTime, Utc}; // NaiveDateTime
use serde::{Deserialize, Serialize};

use super::intervals;
use crate::binance::{self, klines};
use crate::objects::indicators::Indicator;

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

    // Get iterator for the klines in training + validation's close prices
    pub fn get_close_prices_iter(&self) -> Box<impl Iterator<Item = f64> + '_> {
        Box::new(
            self.training
                .iter()
                .map(|kline| kline.close)
                .chain(self.validation.iter().map(|kline| kline.close)),
        )
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

    pub async fn retrieve_klines_simple(
        &mut self,
        symbol: &str,
        interval: &intervals::CryptoInterval,
        limit_minutes: i64,
        training_percentage: f64,
        force_fetch: bool,
    ) -> Result<(), sqlx::Error> {
        binance::klines::retrieve::retrieve_klines_simple(
            self,
            symbol,
            interval,
            limit_minutes,
            training_percentage,
            force_fetch,
        )
        .await
    }

    pub async fn retrieve_extended_klines(
        &mut self,
        indicator: &Indicator,
    ) -> Result<(), sqlx::Error> {
        binance::indicators::retrieve::retrieve_extended_klines(self, indicator).await
    }
}

// --- Tests --- //
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn create_sample_kline(time: DateTime<Utc>) -> Kline {
        Kline {
            open_time: time,
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000.0,
            close_time: time + chrono::Duration::minutes(1),
            quote_asset_volume: 105000.0,
            number_of_trades: 100,
            taker_buy_base_asset_volume: 600.0,
            taker_buy_quote_asset_volume: 63000.0,
        }
    }

    fn create_test_collection() -> KlineCollection {
        let mut collection = KlineCollection::new();
        collection.symbol = String::from("BTCUSDT");
        collection.interval = intervals::CryptoInterval::Int1m;
        collection.training_percentage = 0.7;

        // Add training data
        let base_time = Utc.timestamp_opt(1609459200, 0).unwrap(); // 2021-01-01 00:00:00 UTC
        for i in 0..3 {
            let time = base_time + chrono::Duration::minutes(i);
            collection.training.push(create_sample_kline(time));
        }

        // Add validation data
        for i in 3..5 {
            let time = base_time + chrono::Duration::minutes(i);
            collection.validation.push(create_sample_kline(time));
        }

        // Add past data
        for i in -2..0 {
            let time = base_time + chrono::Duration::minutes(i);
            collection.past.push(create_sample_kline(time));
        }

        collection
    }

    #[test]
    fn test_kline_creation() {
        let time = Utc::now();
        let kline = create_sample_kline(time);

        assert_eq!(kline.open, 100.0);
        assert_eq!(kline.high, 110.0);
        assert_eq!(kline.low, 90.0);
        assert_eq!(kline.close, 105.0);
        assert_eq!(kline.volume, 1000.0);
    }

    #[test]
    fn test_collection_new() {
        let collection = KlineCollection::new();
        assert_eq!(collection.symbol, "");
        assert_eq!(collection.training_percentage, 1.0);
        assert!(collection.training.is_empty());
        assert!(collection.validation.is_empty());
        assert!(collection.past.is_empty());
    }

    #[test]
    fn test_get_normal_index() {
        let collection = create_test_collection();

        // Test training data access
        let first = collection.get(0).unwrap();
        assert_eq!(first.open, 100.0);

        // Test validation data access
        let fourth = collection.get(3).unwrap();
        assert_eq!(fourth.open, 100.0);

        // Test past data access
        let past = collection.get(5).unwrap();
        assert_eq!(past.open, 100.0);

        // Test out of bounds
        assert!(collection.get(-1).is_none());
        assert!(collection.get(7).is_none());
    }

    #[test]
    fn test_get_reverse_index() {
        let collection = create_test_collection();

        // Test validation data access (reverse)
        let last = collection.get_rev(0).unwrap();
        assert_eq!(last.open, 100.0);

        // Test training data access (reverse)
        let training = collection.get_rev(3).unwrap();
        assert_eq!(training.open, 100.0);

        // Test past data access (reverse)
        let past = collection.get_rev(5).unwrap();
        assert_eq!(past.open, 100.0);

        // Test out of bounds
        assert!(collection.get_rev(-1).is_none());
        assert!(collection.get_rev(7).is_none());
    }

    #[test]
    fn test_get_close_prices_iter() {
        let collection = create_test_collection();
        let close_prices: Vec<f64> = collection.get_close_prices_iter().collect();

        assert_eq!(close_prices.len(), 5); // 3 training + 2 validation
        assert!(close_prices.iter().all(|&price| price == 105.0));
    }

    #[test]
    fn test_get_length() {
        let collection = create_test_collection();
        assert_eq!(collection.get_length(), 5); // 3 training + 2 validation
    }

    #[test]
    fn test_get_limit_minutes() {
        let collection = create_test_collection();
        assert_eq!(collection.get_limit_minutes(), 5); // 5 total klines * 1 minute interval
    }

    #[test]
    fn test_get_first_last_times() {
        let collection = create_test_collection();
        let base_time = Utc.timestamp_opt(1609459200, 0).unwrap();

        println!(
            "{:?} {:?} {:?}",
            collection.past, collection.training, collection.validation
        );

        assert_eq!(collection.get_first_open_time(), base_time);
        assert_eq!(
            collection.get_last_open_time(),
            base_time + chrono::Duration::minutes(4)
        );
        assert_eq!(
            collection.get_first_past_open_time(),
            base_time - chrono::Duration::minutes(2)
        );
    }

    #[test]
    fn test_integrity() {
        let collection = create_test_collection();
        collection.check_integrity();
    }
}
