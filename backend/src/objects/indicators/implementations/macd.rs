use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{IndicatorTrait, MovingAverageConvergenceDivergence},
    klines::KlineCollection,
};

impl MovingAverageConvergenceDivergence {
    pub fn new(
        short_period: i32,
        long_period: i32,
        signal_period: i32,
    ) -> MovingAverageConvergenceDivergence {
        MovingAverageConvergenceDivergence {
            short_period,
            long_period,
            signal_period,
            macd_values: Vec::new(),
            signal_values: Vec::new(),
            histogram_values: Vec::new(),
            criteria: Vec::new(),
        }
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

    fn get_missing_indices(&self) -> Vec<i32> {
        // If the macd_values or signal_values or histogram_values are missing (None)
        (0..self.macd_values.len() as i32)
            .filter(|i| {
                self.macd_values[*i as usize].is_none()
                    || self.signal_values[*i as usize].is_none()
                    || self.histogram_values[*i as usize].is_none()
            })
            .collect::<Vec<i32>>()
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![
            &self.macd_values,
            &self.signal_values,
            &self.histogram_values,
        ]
    }

    fn get_criteria(&mut self, klines_collection: &KlineCollection) -> &Vec<Criterion> {
        &self.criteria
    }
}
