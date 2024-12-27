use sqlx::postgres::PgRow;
use sqlx::Row;

use super::super::{BollingerBands, IndicatorTrait};
use crate::objects::klines::KlineCollection;

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

    fn get_missing_indices(&self) -> Vec<i32> {
        // If the upper_band_values or middle_band_values or lower_band_values are missing (None)
        (0..self.upper_band_values.len() as i32)
            .filter(|i| {
                self.upper_band_values[*i as usize].is_none()
                    || self.middle_band_values[*i as usize].is_none()
                    || self.lower_band_values[*i as usize].is_none()
            })
            .collect::<Vec<i32>>()
    }

    fn calculate(&mut self, kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![
            &self.upper_band_values,
            &self.middle_band_values,
            &self.lower_band_values,
        ]
    }
}
