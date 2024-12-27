use sqlx::postgres::PgRow;
use sqlx::Row;

use super::super::{IndicatorTrait, MovingAverage};
use crate::objects::klines::KlineCollection;

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
        match row.get(0) {
            Some(value) => self.values.push(value),
            None => self.values.push(None),
        }
    }

    fn get_missing_indices(&self) -> Vec<i32> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_none())
            .map(|(i, _)| i as i32)
            .collect()
    }

    // Checked and working correctly @26/12/2024 (values, missing before & after)
    fn calculate(&mut self, kline_collection: &KlineCollection) {
        // Calculate the missing values
        for i in self.get_missing_indices() {
            let index = kline_collection.get_length() - 1 - i;
            let mut sum = 0.0;
            for j in 0..self.period {
                // println!("i: {}, j: {}, i+j {}", i, j, i + j);
                sum += kline_collection.get_rev(index + j).unwrap().close;
            }
            // println!("Set at index: {}", *i as usize);
            self.values[i as usize] = Some(sum / self.period as f64);
            // println!(
            //     "gte_rev: ({}) : {:?}",
            //     i,
            //     kline_collection.get_rev(i).unwrap().open_time
            // );
        }
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![&self.values]
    }
}