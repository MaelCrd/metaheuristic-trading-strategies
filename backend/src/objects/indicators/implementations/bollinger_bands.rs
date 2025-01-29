use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{BollingerBands, IndicatorInformation, IndicatorParameter, IndicatorTrait},
    klines::KlineCollection,
};

impl BollingerBands {
    pub fn new(period: i32, deviation: f64) -> BollingerBands {
        BollingerBands {
            period,
            deviation,
            upper_band_values: Vec::new(),
            middle_band_values: Vec::new(),
            lower_band_values: Vec::new(),
            criteria: Vec::new(),
        }
    }

    pub fn information() -> IndicatorInformation {
        IndicatorInformation {
            struct_name: "BollingerBands".to_string(),
            name: "Bollinger Bands".to_string(),
            description: format!(
                "Bollinger Bands (BB) is a volatility indicator that consists of a middle band being an N-period simple moving average (SMA), an upper band at K times an N-period standard deviation above the middle band, and a lower band at K times an N-period standard deviation below the middle band."
            ),
            parameters: vec![
                IndicatorParameter {
                    name: "period".to_string(),
                    description: "The number of periods to use in the calculation.".to_string(),
                    r#type: "integer".to_string(),
                    default: "20".to_string(),
                },
                IndicatorParameter {
                    name: "deviation".to_string(),
                    description: "The number of standard deviations to use in the calculation.".to_string(),
                    r#type: "float".to_string(),
                    default: "2.0".to_string(),
                },
            ],
        }
    }
}

impl IndicatorTrait for BollingerBands {
    fn information(&self) -> IndicatorInformation {
        Self::information()
    }

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

    fn calculate(&mut self, _kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![
            &self.upper_band_values,
            &self.middle_band_values,
            &self.lower_band_values,
        ]
    }

    fn calculate_criteria(&mut self, _kline_collection: &KlineCollection) {}

    fn get_criteria(&self) -> &Vec<Criterion> {
        &self.criteria
    }
}
