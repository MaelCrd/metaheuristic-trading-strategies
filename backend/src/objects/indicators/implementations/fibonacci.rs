use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{FibonacciRetracement, IndicatorInformation, IndicatorParameter, IndicatorTrait},
    klines::KlineCollection,
};

impl FibonacciRetracement {
    pub fn new(period: i32) -> FibonacciRetracement {
        FibonacciRetracement {
            period,
            values: Vec::new(),
            criteria: Vec::new(),
        }
    }

    pub fn information() -> IndicatorInformation {
        IndicatorInformation {
            struct_name: "FibonacciRetracement".to_string(),
            name: "Fibonacci Retracement".to_string(),
            description: "Fibonacci Retracement".to_string(),
            parameters: vec![IndicatorParameter {
                name: "Period".to_string(),
                description: "Period".to_string(),
                r#type: "i32".to_string(),
                default: "14".to_string(),
            }],
        }
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
}
