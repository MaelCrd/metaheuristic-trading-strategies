use serde::{Deserialize, Serialize};

use super::descent::MultiObjectiveDescent;
use super::nsga2::NSGAII;

/// Represents a variable in the optimization problem
#[derive(Clone, Debug)]
pub enum Variable {
    Float(f64),
    Integer(i64),
    Boolean(bool),
}

/// Defines the bounds and type for each variable
#[derive(Clone, Debug)]
pub enum VariableDefinition {
    Float(f64, f64),   // (min, max)
    Integer(i64, i64), // (min, max)
    Boolean,
}

/// Represents a solution in the optimization problem
#[derive(Clone, Debug)]
pub struct Solution {
    pub variables: Vec<Variable>,
    pub objectives: Vec<f64>,
    pub crowding_distance: f64,
    pub rank: usize,
}

impl Solution {
    pub fn new(variables: Vec<Variable>, objectives: Vec<f64>) -> Self {
        Solution {
            variables,
            objectives,
            crowding_distance: 0.0,
            rank: 0,
        }
    }

    pub fn show_short(&self) -> String {
        let vars = self
            .variables
            .iter()
            .map(|v| match v {
                Variable::Float(f) => f.to_string(),
                Variable::Integer(i) => i.to_string(),
                Variable::Boolean(b) => b.to_string(),
            })
            .collect::<Vec<String>>()
            .join(", ");
        let objs = self
            .objectives
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "Variables: [{}], Objectives: [{}], Rank: {}",
            vars, objs, self.rank
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct VariableDefinitionInfo {
    pub name: String,
    pub description: String,
    pub variable_type: String,
    pub bounds: Option<(f64, f64)>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaheuristicInfo {
    pub name: String,
    pub description: String,
    pub parameters: Vec<VariableDefinitionInfo>,
}

#[derive(Clone, Debug)]
pub enum Metaheuristic {
    NSGAII(NSGAII),
    MultiObjectiveDescent(MultiObjectiveDescent),
}

impl Metaheuristic {
    pub fn get_all_info() -> Vec<MetaheuristicInfo> {
        vec![NSGAII::get_info(), MultiObjectiveDescent::get_info()]
    }
}

pub trait MetaheuristicTrait {
    fn run(
        &self,
        num_generations: usize,
        evaluate: impl Fn(&[Variable]) -> Vec<f64> + Clone + Sync + Send,
    ) -> Vec<Solution>;
}

impl MetaheuristicTrait for Metaheuristic {
    fn run(
        &self,
        num_generations: usize,
        evaluate: impl Fn(&[Variable]) -> Vec<f64> + Clone + Sync + Send,
    ) -> Vec<Solution> {
        match self {
            Metaheuristic::MultiObjectiveDescent(simple_descent) => {
                simple_descent.run(num_generations, evaluate)
            }
            Metaheuristic::NSGAII(nsga2) => nsga2.run(num_generations, evaluate),
        }
    }
}
