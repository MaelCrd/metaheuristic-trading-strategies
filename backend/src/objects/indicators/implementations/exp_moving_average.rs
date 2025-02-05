use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{
        ExponentialMovingAverage, IndicatorInformation, IndicatorParameter, IndicatorTrait,
        Variable,
    },
    klines::KlineCollection,
};

impl ExponentialMovingAverage {
    pub fn new(period: i32) -> ExponentialMovingAverage {
        ExponentialMovingAverage {
            period,
            values: Vec::new(),
            criteria: Vec::new(),
            criteria_count: 2,
        }
    }

    pub fn information() -> IndicatorInformation {
        IndicatorInformation {
            struct_name: "ExponentialMovingAverage".to_string(),
            name: "Exponential Moving Average".to_string(),
            description: "Exponential Moving Average".to_string(),
            parameters: vec![IndicatorParameter {
                name: "Period".to_string(),
                description: "The number of periods to use in the calculation.".to_string(),
                r#type: "integer".to_string(),
                default: "20".to_string(),
                min: Some("1".to_string()),
                max: None,
            }],
        }
    }
}

impl IndicatorTrait for ExponentialMovingAverage {
    fn information(&self) -> IndicatorInformation {
        Self::information()
    }

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

    fn get_missing_indices(&self) -> Vec<i32> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_none())
            .map(|(i, _)| i as i32)
            .collect()
    }

    fn calculate(&mut self, _kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![&self.values]
    }

    fn calculate_criteria(&mut self, _kline_collection: &KlineCollection) {}

    fn get_criteria(&self) -> &Vec<Criterion> {
        &self.criteria
    }

    fn get_criteria_count(&self) -> i32 {
        self.criteria_count
    }

    fn clone_with_new_parameters(&self, parameters: &[Variable]) -> Self {
        let period = match parameters[0] {
            Variable::Integer(v) => v,
            _ => panic!("Invalid parameter type"),
        };

        Self::new(period as i32)
    }
}
