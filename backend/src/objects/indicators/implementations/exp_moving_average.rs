use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{ExponentialMovingAverage, IndicatorTrait},
    klines::KlineCollection,
};

impl ExponentialMovingAverage {
    pub fn new(period: i32) -> ExponentialMovingAverage {
        ExponentialMovingAverage {
            period,
            values: Vec::new(),
            criteria: Vec::new(),
        }
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

    fn get_missing_indices(&self) -> Vec<i32> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_none())
            .map(|(i, _)| i as i32)
            .collect()
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![&self.values]
    }

    fn get_criteria(&mut self, klines_collection: &KlineCollection) -> &Vec<Criterion> {
        &self.criteria
    }
}
