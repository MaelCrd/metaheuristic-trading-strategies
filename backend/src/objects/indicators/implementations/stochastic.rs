use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{IndicatorInformation, IndicatorParameter, IndicatorTrait, StochasticOscillator},
    klines::KlineCollection,
};

impl StochasticOscillator {
    pub fn new(k_period: i32, d_period: i32) -> StochasticOscillator {
        StochasticOscillator {
            k_period,
            d_period,
            k_values: Vec::new(),
            d_values: Vec::new(),
            criteria: Vec::new(),
        }
    }

    pub fn information() -> IndicatorInformation {
        IndicatorInformation {
            struct_name: "StochasticOscillator".to_string(),
            name: "Stochastic Oscillator".to_string(),
            description: "Calculates the Stochastic Oscillator indicator".to_string(),
            parameters: vec![
                IndicatorParameter {
                    name: "k_period".to_string(),
                    description: "The period of the K line".to_string(),
                    r#type: "integer".to_string(),
                    default: "14".to_string(),
                },
                IndicatorParameter {
                    name: "d_period".to_string(),
                    description: "The period of the D line".to_string(),
                    r#type: "integer".to_string(),
                    default: "3".to_string(),
                },
            ],
        }
    }
}

impl IndicatorTrait for StochasticOscillator {
    fn column_names(&self) -> Vec<String> {
        vec![
            format!("i_SO_{:?}_{:?}_k", self.k_period, self.d_period).to_lowercase(),
            format!("i_SO_{:?}_{:?}_d", self.k_period, self.d_period).to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.k_period + self.d_period
    }

    fn reserve_space(&mut self, n: i32) {
        self.k_values.reserve(n as usize);
        self.d_values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        match row.get(0) {
            Some(value) => self.k_values.push(Some(value)),
            None => self.k_values.push(None),
        }
        match row.get(1) {
            Some(value) => self.d_values.push(Some(value)),
            None => self.d_values.push(None),
        }
    }

    fn get_missing_indices(&self) -> Vec<i32> {
        // Indices where the k_values or d_values are missing (None)
        (0..self.k_values.len() as i32)
            .filter(|i| {
                self.k_values[*i as usize].is_none() || self.d_values[*i as usize].is_none()
            })
            .collect::<Vec<i32>>()
    }

    // Checked and working correctly @26/12/2024 (values, missing before & after)
    fn calculate(&mut self, kline_collection: &KlineCollection) {
        let kline_collection_length = kline_collection.get_length();

        // Missing indices 2 : The indices where the k_values or d_values are missing (None)
        // do this before extending the k_values
        let missing_indices = self.get_missing_indices();
        // println!("Missing indices {:?}", missing_indices);

        // Extend the k_values from the beginning
        self.k_values
            .splice(0..0, vec![None; self.d_period as usize]);

        // Create a vector of the indices additional for d_values calculation
        let iter_kline = (-self.d_period..0).collect::<Vec<_>>();

        // Calculate the K values
        for i in missing_indices.iter().chain(iter_kline.iter()) {
            // let index = kline_collection_length - 1 - i;
            let index = kline_collection_length - 1 - i;
            let array_index = kline_collection_length + self.d_period - 1 - index;
            // println!("Calculating SO for index: {}", index);
            let mut max = 0.0;
            let mut min = f64::MAX;
            for j in 0..self.k_period {
                let kline = kline_collection.get_rev(index + j).unwrap();
                if kline.high > max {
                    max = kline.high;
                }
                if kline.low < min {
                    min = kline.low;
                }
            }
            self.k_values[array_index as usize] =
                Some((kline_collection.get_rev(index).unwrap().close - min) / (max - min));
        }

        // Calculate the D values
        for i in missing_indices {
            // println!("Calculating SO D for index: {}", i);
            let mut sum = 0.0;
            for j in 0..self.d_period {
                sum += self.k_values[(self.d_period + i - j) as usize].unwrap();
            }
            self.d_values[i as usize] = Some(sum / self.d_period as f64);
        }

        // Remove the +self.period values
        self.k_values.drain(0..self.d_period as usize);
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![&self.k_values, &self.d_values]
    }

    fn calculate_criteria(&mut self, _kline_collection: &KlineCollection) {}

    fn get_criteria(&self) -> &Vec<Criterion> {
        &self.criteria
    }
}
