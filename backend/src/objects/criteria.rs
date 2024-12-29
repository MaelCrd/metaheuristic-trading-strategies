use rayon::prelude::*;
use serde::{Deserialize, Serialize};

// Cross object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCriterion {
    pub values: Vec<bool>,
}

impl CrossCriterion {
    pub fn new(vec1: Box<impl Iterator<Item = f64>>, vec2: Box<impl Iterator<Item = f64>>) -> Self {
        let size = vec1.size_hint().1.unwrap();
        let mut cross_criterion = CrossCriterion {
            values: Vec::with_capacity(size),
        };
        cross_criterion.values.resize(size, false);

        // when vec1 crosses vec2, the value is true, only one time
        let mut above = true;
        for (i, (x, y)) in vec1.zip(vec2).enumerate() {
            if x > y {
                if !above {
                    cross_criterion.values[i] = true;
                    above = true;
                }
            } else {
                above = false;
            }
        }

        cross_criterion
    }
}

// Compare object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareCriterion {
    pub values: Vec<bool>,
}

impl CompareCriterion {
    pub fn new(
        vec1: Box<impl Iterator<Item = f64>>,
        vec2: Box<impl Iterator<Item = f64>>,
    ) -> CompareCriterion {
        let size = vec1.size_hint().1.unwrap();
        let mut compare_criterion = CompareCriterion {
            values: Vec::with_capacity(size),
        };
        compare_criterion.values.resize(size, false);

        for (i, (x, y)) in vec1.zip(vec2).enumerate() {
            compare_criterion.values[i] = x > y;
        }

        compare_criterion
    }
}

// Types of criteria : Cross and Compare
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Criterion {
    Cross(CrossCriterion),
    Compare(CompareCriterion),
}

// Trait for criteria
pub trait CriterionTrait {
    fn get_values(&self) -> &Vec<bool>;
    // fn set_values(&mut self, values: Vec<bool>);

    fn is_empty(&self) -> bool;
}

// Implementing trait for CrossCriterion
impl CriterionTrait for CrossCriterion {
    fn get_values(&self) -> &Vec<bool> {
        &self.values
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// Implementing trait for CompareCriterion
impl CriterionTrait for CompareCriterion {
    fn get_values(&self) -> &Vec<bool> {
        &self.values
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// Implementing trait for Criteria
impl CriterionTrait for Criterion {
    fn get_values(&self) -> &Vec<bool> {
        match self {
            Criterion::Cross(criteria) => criteria.get_values(),
            Criterion::Compare(criteria) => criteria.get_values(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Criterion::Cross(criteria) => criteria.is_empty(),
            Criterion::Compare(criteria) => criteria.is_empty(),
        }
    }
}
