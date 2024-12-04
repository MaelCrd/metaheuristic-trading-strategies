use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Ohlcv {
    timestamp: u32,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn read_csv(file_path: &str) -> Result<Vec<Ohlcv>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Ohlcv = result?;
        records.push(record);
    }
    Ok(records)
}
