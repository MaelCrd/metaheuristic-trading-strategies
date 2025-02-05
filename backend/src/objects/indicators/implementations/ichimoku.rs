use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::objects::{
    criteria::Criterion,
    indicators::{IchimokuCloud, IndicatorInformation, IndicatorParameter, IndicatorTrait},
    klines::KlineCollection,
};

impl IchimokuCloud {
    pub fn new(conversion_period: i32, base_period: i32, lagging_span: i32) -> IchimokuCloud {
        IchimokuCloud {
            conversion_period,
            base_period,
            lagging_span,
            conversion_line_values: Vec::new(),
            base_line_values: Vec::new(),
            lagging_span_values: Vec::new(),
            leading_span_a_values: Vec::new(),
            leading_span_b_values: Vec::new(),
            criteria: Vec::new(),
            criteria_count: 2,
        }
    }

    pub fn information() -> IndicatorInformation {
        IndicatorInformation {
            struct_name: "IchimokuCloud".to_string(),
            name: "Ichimoku Cloud".to_string(),
            description: "Ichimoku Cloud indicator".to_string(),
            parameters: vec![
                IndicatorParameter {
                    name: "Conversion Period".to_string(),
                    description: "The period for the conversion line".to_string(),
                    r#type: "integer".to_string(),
                    default: "9".to_string(),
                    min: Some("1".to_string()),
                    max: None,
                },
                IndicatorParameter {
                    name: "Base Period".to_string(),
                    description: "The period for the base line".to_string(),
                    r#type: "integer".to_string(),
                    default: "26".to_string(),
                    min: Some("1".to_string()),
                    max: None,
                },
                IndicatorParameter {
                    name: "Lagging Span".to_string(),
                    description: "The period for the lagging span".to_string(),
                    r#type: "integer".to_string(),
                    default: "52".to_string(),
                    min: Some("1".to_string()),
                    max: None,
                },
            ],
        }
    }
}

impl IndicatorTrait for IchimokuCloud {
    fn information(&self) -> IndicatorInformation {
        Self::information()
    }

    fn column_names(&self) -> Vec<String> {
        vec![
            format!(
                "i_IC_{:?}_{:?}_{:?}_conversion",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_base",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_lagging",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_leading_a",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
            format!(
                "i_IC_{:?}_{:?}_{:?}_leading_b",
                self.conversion_period, self.base_period, self.lagging_span
            )
            .to_lowercase(),
        ]
    }

    fn n_before_needed(&self) -> i32 {
        self.conversion_period
    }

    fn reserve_space(&mut self, n: i32) {
        self.conversion_line_values.reserve(n as usize);
        self.base_line_values.reserve(n as usize);
        self.lagging_span_values.reserve(n as usize);
        self.leading_span_a_values.reserve(n as usize);
        self.leading_span_b_values.reserve(n as usize);
    }

    fn store_row(&mut self, row: &PgRow) {
        self.conversion_line_values.push(row.get(0));
        self.base_line_values.push(row.get(1));
        self.lagging_span_values.push(row.get(2));
        self.leading_span_a_values.push(row.get(3));
        self.leading_span_b_values.push(row.get(4));
    }

    fn get_missing_indices(&self) -> Vec<i32> {
        // If the conversion_line_values or base_line_values or lagging_span_values
        // or leading_span_a_values or leading_span_b_values are missing (None)
        (0..self.conversion_line_values.len() as i32)
            .filter(|i| {
                self.conversion_line_values[*i as usize].is_none()
                    || self.base_line_values[*i as usize].is_none()
                    || self.lagging_span_values[*i as usize].is_none()
                    || self.leading_span_a_values[*i as usize].is_none()
                    || self.leading_span_b_values[*i as usize].is_none()
            })
            .collect::<Vec<i32>>()
    }

    fn calculate(&mut self, _kline_collection: &KlineCollection) {
        return;
    }

    fn get_values(&self) -> Vec<&Vec<Option<f64>>> {
        vec![
            &self.conversion_line_values,
            &self.base_line_values,
            &self.lagging_span_values,
            &self.leading_span_a_values,
            &self.leading_span_b_values,
        ]
    }

    fn calculate_criteria(&mut self, _kline_collection: &KlineCollection) {}

    fn get_criteria(&self) -> &Vec<Criterion> {
        &self.criteria
    }

    fn get_criteria_count(&self) -> i32 {
        self.criteria_count
    }
}
