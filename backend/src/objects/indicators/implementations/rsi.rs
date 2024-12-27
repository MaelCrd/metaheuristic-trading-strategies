use sqlx::postgres::PgRow;
use sqlx::Row;

use super::super::{IndicatorTrait, RelativeStrengthIndex};
use crate::objects::klines::KlineCollection;

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
}
