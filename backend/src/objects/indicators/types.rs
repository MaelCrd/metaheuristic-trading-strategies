use serde::Serialize;
use sqlx::postgres::PgRow;

use crate::binance;
use crate::metaheuristic::{Variable, VariableDefinition};
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
    // Returns the info
    fn information(&self) -> IndicatorInformation;

    // Returns the variable definitions
    fn get_params_variable_definitions(&self) -> Vec<VariableDefinition> {
        let info = self.information();
        let mut variable_definitions: Vec<VariableDefinition> = Vec::new();
        for parameter in info.parameters {
            variable_definitions.push(match parameter.r#type.as_str() {
                "float" => VariableDefinition::Float(
                    parameter.min.unwrap_or("0".to_string()).parse().unwrap(),
                    parameter.max.unwrap_or("400".to_string()).parse().unwrap(),
                ),
                "integer" => VariableDefinition::Integer(
                    parameter.min.unwrap_or("0".to_string()).parse().unwrap(),
                    parameter.max.unwrap_or("40".to_string()).parse().unwrap(),
                ),
                "boolean" => VariableDefinition::Boolean,
                _ => VariableDefinition::Float(0.0, 0.0),
            });
        }

        variable_definitions
    }

    // Returns all the variable definitions
    fn get_all_variable_definitions(&self) -> Vec<VariableDefinition> {
        let mut variable_definitions = self.get_params_variable_definitions();

        for _ in 0..self.get_criteria_count() {
            variable_definitions.push(VariableDefinition::Boolean);
        }

        variable_definitions
    }

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

    // Get the count of criteria
    fn get_criteria_count(&self) -> i32;

    // Clone but with new parameters
    fn clone_with_new_parameters(&self, parameters: &[Variable]) -> Self;
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

    pub fn new_from_struct_name(
        struct_name: &String,
        json_parameters: &serde_json::Value,
    ) -> Option<Indicator> {
        match struct_name.as_str() {
            "MovingAverage" => {
                let period = json_parameters.get("period");
                if period.is_none() {
                    return None;
                };
                Some(Indicator::MovingAverage(MovingAverage::new(
                    period.unwrap().as_i64().unwrap().try_into().unwrap(),
                )))
            }
            "ExponentialMovingAverage" => {
                let period = json_parameters.get("period");
                if period.is_none() {
                    return None;
                };
                Some(Indicator::ExponentialMovingAverage(
                    ExponentialMovingAverage::new(
                        period.unwrap().as_i64().unwrap().try_into().unwrap(),
                    ),
                ))
            }
            "RelativeStrengthIndex" => {
                let period = json_parameters.get("period");
                if period.is_none() {
                    return None;
                };
                Some(Indicator::RelativeStrengthIndex(
                    RelativeStrengthIndex::new(
                        period.unwrap().as_i64().unwrap().try_into().unwrap(),
                    ),
                ))
            }
            "MovingAverageConvergenceDivergence" => {
                let short_period = json_parameters.get("short_period");
                let long_period = json_parameters.get("long_period");
                let signal_period = json_parameters.get("signal_period");
                if short_period.is_none() || long_period.is_none() || signal_period.is_none() {
                    return None;
                };
                Some(Indicator::MovingAverageConvergenceDivergence(
                    MovingAverageConvergenceDivergence::new(
                        short_period.unwrap().as_i64().unwrap().try_into().unwrap(),
                        long_period.unwrap().as_i64().unwrap().try_into().unwrap(),
                        signal_period.unwrap().as_i64().unwrap().try_into().unwrap(),
                    ),
                ))
            }
            "BollingerBands" => {
                let period = json_parameters.get("period");
                let deviation = json_parameters.get("deviation");
                if period.is_none() || deviation.is_none() {
                    return None;
                };
                Some(Indicator::BollingerBands(BollingerBands::new(
                    period.unwrap().as_i64().unwrap().try_into().unwrap(),
                    deviation.unwrap().as_f64().unwrap(),
                )))
            }
            "FibonacciRetracement" => {
                let period = json_parameters.get("period");
                if period.is_none() {
                    return None;
                };
                Some(Indicator::FibonacciRetracement(FibonacciRetracement::new(
                    period.unwrap().as_i64().unwrap().try_into().unwrap(),
                )))
            }
            "StochasticOscillator" => {
                let k_period = json_parameters.get("k_period");
                let d_period = json_parameters.get("d_period");
                if k_period.is_none() || d_period.is_none() {
                    return None;
                };
                Some(Indicator::StochasticOscillator(StochasticOscillator::new(
                    k_period.unwrap().as_i64().unwrap().try_into().unwrap(),
                    d_period.unwrap().as_i64().unwrap().try_into().unwrap(),
                )))
            }
            "OnBalanceVolume" => {
                let period = json_parameters.get("period");
                if period.is_none() {
                    return None;
                };
                Some(Indicator::OnBalanceVolume(OnBalanceVolume::new(
                    period.unwrap().as_i64().unwrap().try_into().unwrap(),
                )))
            }
            "IchimokuCloud" => {
                let conversion_period = json_parameters.get("conversion_period");
                let base_period = json_parameters.get("base_period");
                let lagging_span = json_parameters.get("lagging_span");
                if conversion_period.is_none() || base_period.is_none() || lagging_span.is_none() {
                    return None;
                };
                Some(Indicator::IchimokuCloud(IchimokuCloud::new(
                    conversion_period
                        .unwrap()
                        .as_i64()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                    base_period.unwrap().as_i64().unwrap().try_into().unwrap(),
                    lagging_span.unwrap().as_i64().unwrap().try_into().unwrap(),
                )))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IndicatorParameter {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub default: String,
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndicatorInformation {
    pub struct_name: String,
    pub name: String,
    pub description: String,
    pub parameters: Vec<IndicatorParameter>,
}

// Indicators

#[derive(Debug, Clone)]
pub struct MovingAverage {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria_count: i32,
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct ExponentialMovingAverage {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria_count: i32,
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria_count: i32,
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
    pub criteria_count: i32,
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
    pub criteria_count: i32,
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct FibonacciRetracement {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria_count: i32,
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
    pub criteria_count: i32,
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone)]
pub struct OnBalanceVolume {
    // Parameters
    pub period: i32,
    // Values
    pub values: Vec<Option<f64>>,
    // Criteria
    pub criteria_count: i32,
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
    pub criteria_count: i32,
    pub criteria: Vec<Criterion>,
}
