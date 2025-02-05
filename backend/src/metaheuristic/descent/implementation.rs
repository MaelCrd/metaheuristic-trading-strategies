use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::metaheuristic::objects::{
    MetaheuristicInfo, MetaheuristicTrait, Solution, Variable, VariableDefinition,
    VariableDefinitionInfo,
};
use crate::objects::{indicators::Indicator, klines::KlineCollection};

#[derive(Clone, Debug)]
pub struct MultiObjectiveDescent {
    step_size: f64,
    variable_definitions: Vec<VariableDefinition>,
    max_iterations_without_improvement: usize,
    archive_size: usize,
    num_objectives: usize,
}

impl MultiObjectiveDescent {
    pub fn new(
        step_size: f64,
        variable_definitions: Vec<VariableDefinition>,
        max_iterations_without_improvement: usize,
        archive_size: usize,
        num_objectives: usize,
    ) -> Self {
        MultiObjectiveDescent {
            step_size,
            variable_definitions,
            max_iterations_without_improvement,
            archive_size,
            num_objectives,
        }
    }

    pub fn new_from_json(
        json: &serde_json::Value,
        variable_definitions: Vec<VariableDefinition>,
        num_objectives: usize,
    ) -> Result<Self, String> {
        let step_size_option = json.get("step_size");
        let archive_size_option = json.get("archive_size");
        if step_size_option.is_none() || archive_size_option.is_none() {
            return Err("Missing parameters for the algorithm".to_string());
        }

        let step_size = step_size_option.unwrap().as_str().unwrap().parse::<f64>();
        let archive_size = archive_size_option
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<usize>();
        if !step_size.is_ok() || !archive_size.is_ok() {
            return Err("Invalid parameters for the algorithm".to_string());
        }

        Ok(MultiObjectiveDescent {
            step_size: step_size.unwrap(),
            variable_definitions: variable_definitions,
            max_iterations_without_improvement: 1000,
            archive_size: archive_size.unwrap(),
            num_objectives: num_objectives,
        })
    }

    pub fn get_info() -> MetaheuristicInfo {
        MetaheuristicInfo {
            name: "Multi-Objective Simple Descent".to_string(),
            description: "A descent algorithm adapted for multi-objective optimization".to_string(),
            parameters: vec![
                VariableDefinitionInfo {
                    name: "step_size".to_string(),
                    description: "Initial step size for variable perturbation".to_string(),
                    variable_type: "float".to_string(),
                    bounds: Some((0.0, f64::INFINITY)),
                },
                VariableDefinitionInfo {
                    name: "archive_size".to_string(),
                    description: "Maximum size of the non-dominated solutions archive".to_string(),
                    variable_type: "integer".to_string(),
                    bounds: Some((1.0, f64::INFINITY)),
                },
                VariableDefinitionInfo {
                    name: "num_objectives".to_string(),
                    description: "Number of objectives to optimize".to_string(),
                    variable_type: "integer".to_string(),
                    bounds: Some((2.0, f64::INFINITY)),
                },
            ],
        }
    }

    fn initialize_solution(&self) -> Solution {
        let mut rng = rand::thread_rng();
        let variables: Vec<Variable> = self
            .variable_definitions
            .iter()
            .map(|def| match def {
                VariableDefinition::Float(min, max) => Variable::Float(rng.gen_range(*min..*max)),
                VariableDefinition::Integer(min, max) => {
                    Variable::Integer(rng.gen_range(*min..=*max))
                }
                VariableDefinition::Boolean => Variable::Boolean(rng.gen_bool(0.5)),
            })
            .collect();

        Solution::new(variables, vec![0.0; self.num_objectives])
    }

    fn generate_neighbor(&self, current: &Solution) -> Solution {
        let mut rng = rand::thread_rng();
        let mut neighbor = current.clone();

        // Randomly select one variable to modify
        let var_idx = rng.gen_range(0..self.variable_definitions.len());

        match (
            &self.variable_definitions[var_idx],
            &current.variables[var_idx],
        ) {
            (VariableDefinition::Float(min, max), Variable::Float(val)) => {
                let perturbation = (rng.gen::<f64>() * 2.0 - 1.0) * self.step_size;
                let new_val = (val + perturbation).clamp(*min, *max);
                neighbor.variables[var_idx] = Variable::Float(new_val);
            }
            (VariableDefinition::Integer(min, max), Variable::Integer(val)) => {
                let int_step = (self.step_size.round() as i64).max(1);
                let perturbation = rng.gen_range(-int_step..=int_step);
                let new_val = (*val + perturbation).clamp(*min, *max);
                neighbor.variables[var_idx] = Variable::Integer(new_val);
            }
            (VariableDefinition::Boolean, Variable::Boolean(val)) => {
                neighbor.variables[var_idx] = Variable::Boolean(!val);
            }
            _ => panic!("Variable type mismatch"),
        }

        neighbor
    }

    fn dominates(&self, solution1: &Solution, solution2: &Solution) -> bool {
        let mut one_is_better = false;

        for i in 0..self.num_objectives {
            if solution1.objectives[i] > solution2.objectives[i] {
                return false;
            }
            if solution1.objectives[i] < solution2.objectives[i] {
                one_is_better = true;
            }
        }

        one_is_better
    }

    fn update_archive(&self, archive: &mut Vec<Solution>, new_solution: Solution) {
        // Check if the new solution is dominated by any archive solution
        if !archive.iter().any(|sol| self.dominates(sol, &new_solution)) {
            // Remove solutions that are dominated by the new solution
            archive.retain(|sol| !self.dominates(&new_solution, sol));

            // Add new solution to archive
            archive.push(new_solution);

            // If archive exceeds maximum size, remove solutions using crowding distance
            if archive.len() > self.archive_size {
                self.trim_archive(archive);
            }
        }
    }

    fn calculate_crowding_distance(&self, solutions: &mut Vec<Solution>) {
        let n = solutions.len();
        if n <= 2 {
            for sol in solutions.iter_mut() {
                sol.crowding_distance = f64::INFINITY;
            }
            return;
        }

        // Initialize distances
        for sol in solutions.iter_mut() {
            sol.crowding_distance = 0.0;
        }

        // Calculate crowding distance for each objective
        for m in 0..self.num_objectives {
            // Sort solutions based on current objective
            solutions.sort_by(|a, b| {
                a.objectives[m]
                    .partial_cmp(&b.objectives[m])
                    .unwrap_or(Ordering::Equal)
            });

            // Set boundary points to infinity
            solutions[0].crowding_distance = f64::INFINITY;
            solutions[n - 1].crowding_distance = f64::INFINITY;

            // Calculate distance for intermediate points
            let obj_range = solutions[n - 1].objectives[m] - solutions[0].objectives[m];
            if obj_range > 0.0 {
                for i in 1..n - 1 {
                    solutions[i].crowding_distance += (solutions[i + 1].objectives[m]
                        - solutions[i - 1].objectives[m])
                        / obj_range;
                }
            }
        }
    }

    fn trim_archive(&self, archive: &mut Vec<Solution>) {
        // Calculate crowding distances
        self.calculate_crowding_distance(archive);

        // Sort by crowding distance (descending)
        archive.sort_by(|a, b| {
            b.crowding_distance
                .partial_cmp(&a.crowding_distance)
                .unwrap()
        });

        // Keep only the desired number of solutions
        archive.truncate(self.archive_size);
    }

    pub fn run<F>(
        &self,
        max_iterations: usize,
        evaluate: F,
        kline_collections: &Vec<KlineCollection>,
        indicators: &Vec<Indicator>,
        variable_definitions_sep: &Vec<Vec<VariableDefinition>>,
    ) -> Vec<Solution>
    where
        F: Fn(
            &[Variable],
            &Vec<KlineCollection>,
            &Vec<Indicator>,
            &Vec<Vec<VariableDefinition>>,
        ) -> Vec<f64>,
    {
        let mut rng = rand::thread_rng();
        let mut archive: Vec<Solution> = Vec::new();

        // Initialize first solution
        let mut current = self.initialize_solution();
        current.objectives = evaluate(
            &current.variables,
            kline_collections,
            indicators,
            variable_definitions_sep,
        );
        self.update_archive(&mut archive, current.clone());

        let mut iterations_without_improvement = 0;

        for _ in 0..max_iterations {
            // Generate and evaluate neighbor
            let mut neighbor = self.generate_neighbor(&current);
            neighbor.objectives = evaluate(
                &neighbor.variables,
                kline_collections,
                indicators,
                variable_definitions_sep,
            );

            // Update archive and check for improvement
            let archive_size_before = archive.len();
            self.update_archive(&mut archive, neighbor.clone());

            // Update current solution if neighbor is non-dominated
            if !self.dominates(&current, &neighbor) {
                current = neighbor;

                // Check if archive improved
                if archive.len() > archive_size_before {
                    iterations_without_improvement = 0;
                } else {
                    iterations_without_improvement += 1;
                }
            } else {
                iterations_without_improvement += 1;
            }

            // Early stopping if no improvement for too long
            if iterations_without_improvement >= self.max_iterations_without_improvement {
                break;
            }
        }

        archive
    }
}

// Implement the MetaheuristicTrait for MultiObjectiveDescent
impl MetaheuristicTrait for MultiObjectiveDescent {
    fn run(
        &self,
        num_generations: usize,
        evaluate: impl Fn(
                &[Variable],
                &Vec<KlineCollection>,
                &Vec<Indicator>,
                &Vec<Vec<VariableDefinition>>,
            ) -> Vec<f64>
            + Clone
            + Sync
            + Send,
        kline_collections: &Vec<KlineCollection>,
        indicators: &Vec<Indicator>,
        variable_definitions_sep: &Vec<Vec<VariableDefinition>>,
    ) -> Vec<Solution> {
        self.run(
            num_generations,
            evaluate,
            kline_collections,
            indicators,
            variable_definitions_sep,
        )
    }
}
