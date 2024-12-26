// --- Indicators --- //

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

use super::klines::KlineCollection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverage {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialMovingAverage {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeStrengthIndex {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverageConvergenceDivergence {
    // Parameters
    pub short_period: i32,
    pub long_period: i32,
    pub signal_period: i32,
    // Values
    pub macd_values: Vec<f64>,
    pub signal_values: Vec<f64>,
    pub histogram_values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerBands {
    // Parameters
    pub period: i32,
    pub deviation: f64,
    // Values
    pub upper_band_values: Vec<f64>,
    pub middle_band_values: Vec<f64>,
    pub lower_band_values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibonacciRetracement {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StochasticOscillator {
    // Parameters
    pub k_period: i32,
    pub d_period: i32,
    // Values
    pub k_values: Vec<f64>,
    pub d_values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnBalanceVolume {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IchimokuCloud {
    // Parameters
    pub conversion_period: i32,
    pub base_period: i32,
    pub lagging_span: i32,
    // Values
    pub conversion_line_values: Vec<f64>,
    pub base_line_values: Vec<f64>,
    pub lagging_span_values: Vec<f64>,
    pub leading_span_a_values: Vec<f64>,
    pub leading_span_b_values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Indicator {
    MovingAverage(MovingAverage),
    ExponentialMovingAverage(ExponentialMovingAverage),
    RelativeStrengthIndex(RelativeStrengthIndex),
    MovingAverageConvergenceDivergence(MovingAverageConvergenceDivergence),
    BollingerBands(BollingerBands),
    FibonacciRetracement(FibonacciRetracement),
    StochasticOscillator(StochasticOscillator),
    OnBalanceVolume(OnBalanceVolume),
    IchimokuCloud(IchimokuCloud),
}

pub trait IndicatorTrait {
    // Returns the column names of the indicator
    fn column_names(&self) -> Vec<String>;

    // Returns the number of rows needed for the indicator to be computed on the first row
    fn n_before_needed(&self) -> i32;

    // Reserve space for the values
    fn reserve_space(&mut self, n: i32);

    // Store a row in the indicator
    fn store_row(&mut self, row: &PgRow);

    // Store the rows in the indicator
    fn store_rows(&mut self, rows: &Vec<PgRow>) {
        self.reserve_space(rows.len() as i32);

        for row in rows {
            self.store_row(row);
        }
    }

    // Calculates the indicator values
    fn calculate(&mut self, kline_collection: &KlineCollection);

    // Get the values of the indicator
    fn get_values(&self) -> Vec<&Vec<f64>>;

    // // Get values length
    // fn get_values_length(&self) -> i32 {
    //     self.get_values()[0].len() as i32
    // }
}

impl IndicatorTrait for MovingAverage {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_MA_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }

    fn reserve_space(&mut self, n: i32) {
        self.values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.values.push(row.get(0));
    }

    // Checked and working correctly
    fn calculate(&mut self, kline_collection: &KlineCollection) {
        self.reserve_space(kline_collection.get_length());
        for i in 0..kline_collection.get_length() {
            let mut sum = 0.0;
            for j in 0..self.period {
                // println!("i: {}, j: {}, i+j {}", i, j, i + j);
                sum += kline_collection.get_rev(i + j).unwrap().close;
            }
            self.values.push(sum / self.period as f64);
            // println!(
            //     "gte_rev: ({}) : {:?}",
            //     i,
            //     kline_collection.get_rev(i).unwrap().open_time
            // );
        }
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![&self.values]
    }
}

impl IndicatorTrait for ExponentialMovingAverage {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_EMA_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }

    fn reserve_space(&mut self, n: i32) {
        self.values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.values.push(row.get(0));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![&self.values]
    }
}

impl IndicatorTrait for RelativeStrengthIndex {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_RSI_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }

    fn reserve_space(&mut self, n: i32) {
        self.values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.values.push(row.get(0));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![&self.values]
    }
}

impl IndicatorTrait for MovingAverageConvergenceDivergence {
    fn column_names(&self) -> Vec<String> {
        vec![
            format!(
                "i_MACD_{:?}_{:?}_{:?}_macd",
                self.short_period, self.long_period, self.signal_period
            )
            .to_lowercase(),
            format!(
                "i_MACD_{:?}_{:?}_{:?}_signal",
                self.short_period, self.long_period, self.signal_period
            )
            .to_lowercase(),
            format!(
                "i_MACD_{:?}_{:?}_{:?}_histogram",
                self.short_period, self.long_period, self.signal_period
            )
            .to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.long_period
    }

    fn reserve_space(&mut self, n: i32) {
        self.macd_values.reserve(n as usize);
        self.signal_values.reserve(n as usize);
        self.histogram_values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.macd_values.push(row.get(0));
        self.signal_values.push(row.get(1));
        self.histogram_values.push(row.get(2));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![
            &self.macd_values,
            &self.signal_values,
            &self.histogram_values,
        ]
    }
}

impl IndicatorTrait for BollingerBands {
    fn column_names(&self) -> Vec<String> {
        vec![
            format!("i_BB_{:?}_{:?}_upper", self.period, self.deviation).to_lowercase(),
            format!("i_BB_{:?}_{:?}_middle", self.period, self.deviation).to_lowercase(),
            format!("i_BB_{:?}_{:?}_lower", self.period, self.deviation).to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }

    fn reserve_space(&mut self, n: i32) {
        self.upper_band_values.reserve(n as usize);
        self.middle_band_values.reserve(n as usize);
        self.lower_band_values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.upper_band_values.push(row.get(0));
        self.middle_band_values.push(row.get(1));
        self.lower_band_values.push(row.get(2));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![
            &self.upper_band_values,
            &self.middle_band_values,
            &self.lower_band_values,
        ]
    }
}

impl IndicatorTrait for FibonacciRetracement {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_FR_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }

    fn reserve_space(&mut self, n: i32) {
        self.values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.values.push(row.get(0));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![&self.values]
    }
}

impl IndicatorTrait for StochasticOscillator {
    fn column_names(&self) -> Vec<String> {
        vec![
            format!("i_SO_{:?}_{:?}_k", self.k_period, self.d_period).to_lowercase(),
            format!("i_SO_{:?}_{:?}_d", self.k_period, self.d_period).to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.k_period + self.d_period
    }

    fn reserve_space(&mut self, n: i32) {
        self.k_values.reserve(n as usize);
        self.d_values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.k_values.push(row.get(0));
        self.d_values.push(row.get(1));
    }

    // Checked and working correctly
    fn calculate(&mut self, kline_collection: &KlineCollection) {
        let kline_collection_length = kline_collection.get_length();
        self.reserve_space(kline_collection_length);
        // Calculate the K values
        for i in 0..kline_collection_length + self.d_period {
            let mut max = 0.0;
            let mut min = f64::MAX;
            for j in 0..self.k_period {
                let kline = kline_collection.get_rev(i + j).unwrap();
                if kline.high > max {
                    max = kline.high;
                }
                if kline.low < min {
                    min = kline.low;
                }
            }
            self.k_values
                .push((kline_collection.get_rev(i).unwrap().close - min) / (max - min));
        }

        // Calculate the D values
        for i in 0..kline_collection_length {
            let mut sum = 0.0;
            for j in 0..self.d_period {
                sum += self.k_values[(i + j) as usize];
            }
            self.d_values.push(sum / self.d_period as f64);
        }

        // Remove the +self.period values
        self.k_values.drain(kline_collection_length as usize..);
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![&self.k_values, &self.d_values]
    }
}

impl IndicatorTrait for OnBalanceVolume {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_OBV_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }

    fn reserve_space(&mut self, n: i32) {
        self.values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.values.push(row.get(0));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![&self.values]
    }
}

impl IndicatorTrait for IchimokuCloud {
    fn column_names(&self) -> Vec<String> {
        vec![
            format!(
                "i_IC_{:?}_{:?}_{:?}_conversion",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_base",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_lagging",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_leading_a",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_leading_b",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.conversion_period
    }

    fn reserve_space(&mut self, n: i32) {
        self.conversion_line_values.reserve(n as usize);
        self.base_line_values.reserve(n as usize);
        self.lagging_span_values.reserve(n as usize);
        self.leading_span_a_values.reserve(n as usize);
        self.leading_span_b_values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.conversion_line_values.push(row.get(0));
        self.base_line_values.push(row.get(1));
        self.lagging_span_values.push(row.get(2));
        self.leading_span_a_values.push(row.get(3));
        self.leading_span_b_values.push(row.get(4));
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        vec![
            &self.conversion_line_values,
            &self.base_line_values,
            &self.lagging_span_values,
            &self.leading_span_a_values,
            &self.leading_span_b_values,
        ]
    }
}

impl IndicatorTrait for Indicator {
    fn column_names(&self) -> Vec<String> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.column_names(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.column_names(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.column_names(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.column_names(),
            Indicator::BollingerBands(indicator) => indicator.column_names(),
            Indicator::FibonacciRetracement(indicator) => indicator.column_names(),
            Indicator::StochasticOscillator(indicator) => indicator.column_names(),
            Indicator::OnBalanceVolume(indicator) => indicator.column_names(),
            Indicator::IchimokuCloud(indicator) => indicator.column_names(),
        }
    }

    fn n_before_needed(&self) -> i32 {
        match self {
            Indicator::MovingAverage(indicator) => indicator.n_before_needed(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.n_before_needed(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.n_before_needed(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.n_before_needed(),
            Indicator::BollingerBands(indicator) => indicator.n_before_needed(),
            Indicator::FibonacciRetracement(indicator) => indicator.n_before_needed(),
            Indicator::StochasticOscillator(indicator) => indicator.n_before_needed(),
            Indicator::OnBalanceVolume(indicator) => indicator.n_before_needed(),
            Indicator::IchimokuCloud(indicator) => indicator.n_before_needed(),
        }
    }

    fn reserve_space(&mut self, n: i32) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.reserve_space(n),
            Indicator::ExponentialMovingAverage(indicator) => indicator.reserve_space(n),
            Indicator::RelativeStrengthIndex(indicator) => indicator.reserve_space(n),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.reserve_space(n),
            Indicator::BollingerBands(indicator) => indicator.reserve_space(n),
            Indicator::FibonacciRetracement(indicator) => indicator.reserve_space(n),
            Indicator::StochasticOscillator(indicator) => indicator.reserve_space(n),
            Indicator::OnBalanceVolume(indicator) => indicator.reserve_space(n),
            Indicator::IchimokuCloud(indicator) => indicator.reserve_space(n),
        }
    }

    fn store_row(&mut self, row: &PgRow) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.store_row(row),
            Indicator::ExponentialMovingAverage(indicator) => indicator.store_row(row),
            Indicator::RelativeStrengthIndex(indicator) => indicator.store_row(row),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.store_row(row),
            Indicator::BollingerBands(indicator) => indicator.store_row(row),
            Indicator::FibonacciRetracement(indicator) => indicator.store_row(row),
            Indicator::StochasticOscillator(indicator) => indicator.store_row(row),
            Indicator::OnBalanceVolume(indicator) => indicator.store_row(row),
            Indicator::IchimokuCloud(indicator) => indicator.store_row(row),
        }
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        match self {
            Indicator::MovingAverage(indicator) => indicator.calculate(kline_collection),
            Indicator::ExponentialMovingAverage(indicator) => indicator.calculate(kline_collection),
            Indicator::RelativeStrengthIndex(indicator) => indicator.calculate(kline_collection),
            Indicator::MovingAverageConvergenceDivergence(indicator) => {
                indicator.calculate(kline_collection)
            }
            Indicator::BollingerBands(indicator) => indicator.calculate(kline_collection),
            Indicator::FibonacciRetracement(indicator) => indicator.calculate(kline_collection),
            Indicator::StochasticOscillator(indicator) => indicator.calculate(kline_collection),
            Indicator::OnBalanceVolume(indicator) => indicator.calculate(kline_collection),
            Indicator::IchimokuCloud(indicator) => indicator.calculate(kline_collection),
        }
    }

    fn get_values(&self) -> Vec<&Vec<f64>> {
        match self {
            Indicator::MovingAverage(indicator) => indicator.get_values(),
            Indicator::ExponentialMovingAverage(indicator) => indicator.get_values(),
            Indicator::RelativeStrengthIndex(indicator) => indicator.get_values(),
            Indicator::MovingAverageConvergenceDivergence(indicator) => indicator.get_values(),
            Indicator::BollingerBands(indicator) => indicator.get_values(),
            Indicator::FibonacciRetracement(indicator) => indicator.get_values(),
            Indicator::StochasticOscillator(indicator) => indicator.get_values(),
            Indicator::OnBalanceVolume(indicator) => indicator.get_values(),
            Indicator::IchimokuCloud(indicator) => indicator.get_values(),
        }
    }
}
