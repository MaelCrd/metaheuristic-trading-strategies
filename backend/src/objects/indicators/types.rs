use serde::Serialize;
use sqlx::postgres::PgRow;

use crate::binance;
use crate::objects::{criteria::Criterion, klines::KlineCollection};

#[derive(Debug, Clone)]
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

    // Get missing indices
    fn get_missing_indices(&self) -> Vec<i32>;

    // Calculates the indicator values
    fn calculate(&mut self, kline_collection: &KlineCollection);

    // Get the values of the indicator
    fn get_values(&self) -> Vec<&Vec<Option<f64>>>;

    // Calculate the criteria of the indicator
    fn calculate_criteria(&mut self, kline_collection: &KlineCollection);

    // Get the criteria of the indicator
    fn get_criteria(&self) -> &Vec<Criterion>;
}

impl Indicator {
    pub async fn retrieve(
        &mut self,
        kline_collection: &KlineCollection,
    ) -> Result<(), sqlx::Error> {
        binance::indicators::retrieve::retrieve_indicator(self, &kline_collection).await
    }

    pub fn get_all_indicators_info() -> Vec<IndicatorInformation> {
        vec![
            MovingAverage::information(),
            ExponentialMovingAverage::information(),
            RelativeStrengthIndex::information(),
            MovingAverageConvergenceDivergence::information(),
            BollingerBands::information(),
            FibonacciRetracement::information(),
            StochasticOscillator::information(),
            OnBalanceVolume::information(),
            IchimokuCloud::information(),
        ]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IndicatorParameter {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub default: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndicatorInformation {
    pub struct_name: String,
    pub name: String,
    pub description: String,
    pub parameters: Vec<IndicatorParameter>,
}

#[derive(Debug, Clone)]
pub struct MovingAverage {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct ExponentialMovingAverage {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct MovingAverageConvergenceDivergence {
    // Parameters
    pub short_period: i32,
    pub long_period: i32,
    pub signal_period: i32,
    // Values
    pub macd_values: Vec<Option<f64>>,
    pub signal_values: Vec<Option<f64>>,
    pub histogram_values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct BollingerBands {
    // Parameters
    pub period: i32,
    pub deviation: f64,
    // Values
    pub upper_band_values: Vec<Option<f64>>,
    pub middle_band_values: Vec<Option<f64>>,
    pub lower_band_values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct FibonacciRetracement {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct StochasticOscillator {
    // Parameters
    pub k_period: i32,
    pub d_period: i32,
    // Values
    pub k_values: Vec<Option<f64>>,
    pub d_values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct OnBalanceVolume {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct IchimokuCloud {
    // Parameters
    pub conversion_period: i32,
    pub base_period: i32,
    pub lagging_span: i32,
    // Values
    pub conversion_line_values: Vec<Option<f64>>,
    pub base_line_values: Vec<Option<f64>>,
    pub lagging_span_values: Vec<Option<f64>>,
    pub leading_span_a_values: Vec<Option<f64>>,
    pub leading_span_b_values: Vec<Option<f64>>,
    // Criteria
    pub criteria: Vec<Criterion>,
}
