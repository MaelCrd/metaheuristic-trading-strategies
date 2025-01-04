use rayon::prelude::*;
use serde::{Deserialize, Serialize};

// Cross object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCriterion {
    values: Vec<bool>,
}

impl CrossCriterion {
    pub fn new(vec1: Box<impl Iterator<Item = f64>>, vec2: Box<impl Iterator<Item = f64>>) -> Self {
        panic!("Not implemented");
        // let size = vec1.size_hint().1.unwrap();
        // let mut cross_criterion = CrossCriterion {
        //     values: Vec::with_capacity(size),
        // };
        // cross_criterion.values.resize(size, false);

        // // when vec1 crosses vec2, the value is true, only one time
        // let mut above = true;
        // for (i, (x, y)) in vec1.zip(vec2).enumerate() {
        //     if x > y {
        //         if !above {
        //             cross_criterion.values[i] = true;
        //             above = true;
        //         }
        //     } else {
        //         above = false;
        //     }
        // }

        // cross_criterion
    }

    pub fn new_from(compare_criterion: &Criterion, cross_side: bool) -> CrossCriterion {
        let compare_criterion_values = compare_criterion.get_values();
        let size = compare_criterion_values.len();
        let mut cross_criterion = CrossCriterion {
            values: Vec::with_capacity(size),
        };
        cross_criterion.values.resize(size, false);

        for i in 1..size {
            // If cross_side is true, then the value is true when cross_criterion goes from false to true
            // If cross_side is false, then the value is true when cross_criterion goes from true to false
            cross_criterion.values[i] = match cross_side {
                true => !compare_criterion_values[i - 1] && compare_criterion_values[i],
                false => compare_criterion_values[i - 1] && !compare_criterion_values[i],
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

//// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_criterion_new() {
        let vec1 = Box::new(vec![1.0, 2.0, 3.0, 3.5, 4.0].into_iter());
        let vec2 = Box::new(vec![3.0, 2.5, 1.0, 2.0, 5.0].into_iter());
        let compare_criterion = CompareCriterion::new(vec1, vec2);
        assert_eq!(
            compare_criterion.values,
            vec![false, false, true, true, false]
        );
    }

    #[test]
    fn test_cross_criterion_new_from_true() {
        let compare_criterion = CompareCriterion {
            values: vec![false, false, true, true, false, true],
        };
        let cross_criterion =
            CrossCriterion::new_from(&Criterion::Compare(compare_criterion), true);
        assert_eq!(
            cross_criterion.values,
            vec![false, false, true, false, false, true]
        );
    }

    #[test]
    fn test_cross_criterion_new_from_false() {
        let compare_criterion = CompareCriterion {
            values: vec![false, false, true, true, false, true],
        };
        let cross_criterion =
            CrossCriterion::new_from(&Criterion::Compare(compare_criterion), false);
        assert_eq!(
            cross_criterion.values,
            vec![false, false, false, false, true, false]
        );
    }

    #[test]
    fn test_criterion_trait_for_cross() {
        let cross_criterion = CrossCriterion {
            values: vec![true, false, true, true, false],
        };
        assert_eq!(
            cross_criterion.get_values(),
            &vec![true, false, true, true, false]
        );
        assert!(!cross_criterion.is_empty());
    }

    #[test]
    fn test_criterion_trait_for_compare() {
        let compare_criterion = CompareCriterion {
            values: vec![false, true, false],
        };
        assert_eq!(compare_criterion.get_values(), &vec![false, true, false]);
        assert!(!compare_criterion.is_empty());
    }

    #[test]
    fn test_criterion_trait_for_criterion() {
        let criterion = Criterion::Cross(CrossCriterion {
            values: vec![true, false, true],
        });
        assert_eq!(criterion.get_values(), &vec![true, false, true]);
        assert!(!criterion.is_empty());

        let criterion = Criterion::Compare(CompareCriterion {
            values: vec![false, true, false],
        });
        assert_eq!(criterion.get_values(), &vec![false, true, false]);
        assert!(!criterion.is_empty());
    }
}
