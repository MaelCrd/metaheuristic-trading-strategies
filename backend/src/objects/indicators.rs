// --- Indicators --- //

use serde::{Deserialize, Serialize};

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
    pub period: i32,
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
}

impl IndicatorTrait for MovingAverage {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_MA_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }
}

impl IndicatorTrait for ExponentialMovingAverage {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_EMA_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }
}

impl IndicatorTrait for RelativeStrengthIndex {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_RSI_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
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
}

impl IndicatorTrait for FibonacciRetracement {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_FR_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }
}

impl IndicatorTrait for StochasticOscillator {
    fn column_names(&self) -> Vec<String> {
        vec![
            format!("i_SO_{:?}_k", self.period).to_lowercase(),
            format!("i_SO_{:?}_d", self.period).to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
    }
}

impl IndicatorTrait for OnBalanceVolume {
    fn column_names(&self) -> Vec<String> {
        vec![format!("i_OBV_{:?}", self.period).to_lowercase()]
    }

    fn n_before_needed(&self) -> i32 {
        self.period
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
}
