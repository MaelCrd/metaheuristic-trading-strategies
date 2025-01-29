use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::metaheuristic::objects::{
    MetaheuristicInfo, MetaheuristicTrait, Solution, Variable, VariableDefinition,
    VariableDefinitionInfo,
};

// Parallelize the NSGA-II algorithm using Rayon
use rayon::prelude::*;
// use std::sync::Arc;

/// The main NSGA-II algorithm implementation
#[derive(Clone, Debug)]
pub struct NSGAII {
    population_size: usize,
    variable_definitions: Vec<VariableDefinition>,
    num_objectives: usize,
    mutation_rate: f64,
    crossover_rate: f64,
}

impl NSGAII {
    pub fn new(
        population_size: usize,
        variable_definitions: Vec<VariableDefinition>,
        num_objectives: usize,
        mutation_rate: f64,
        crossover_rate: f64,
    ) -> Self {
        NSGAII {
            population_size,
            variable_definitions,
            num_objectives,
            mutation_rate,
            crossover_rate,
        }
    }

    pub fn new_from_json(
        json: &serde_json::Value,
        variable_definitions: Vec<VariableDefinition>,
        num_objectives: usize,
    ) -> Result<Self, String> {
        let population_size_option = json.get("population_size");
        let mutation_rate_option = json.get("mutation_rate");
        let crossover_rate_option = json.get("crossover_rate");
        if population_size_option.is_none()
            || mutation_rate_option.is_none()
            || crossover_rate_option.is_none()
        {
            return Err("Missing parameters for the algorithm".to_string());
        }

        let population_size = population_size_option
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<usize>();
        let mutation_rate = mutation_rate_option
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<f64>();
        let crossover_rate = crossover_rate_option
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<f64>();
        if !population_size.is_ok() || !mutation_rate.is_ok() || !crossover_rate.is_ok() {
            return Err("Invalid parameters for the algorithm".to_string());
        }

        Ok(NSGAII {
            population_size: population_size.unwrap(),
            variable_definitions: variable_definitions,
            num_objectives: num_objectives,
            mutation_rate: mutation_rate.unwrap(),
            crossover_rate: crossover_rate.unwrap(),
        })
    }

    pub fn get_info() -> MetaheuristicInfo {
        MetaheuristicInfo {
            name: "NSGA-II".to_string(),
            description: "Non-dominated Sorting Genetic Algorithm II".to_string(),
            parameters: vec![
                VariableDefinitionInfo {
                    name: "population_size".to_string(),
                    description: "Number of solutions in the population".to_string(),
                    variable_type: "integer".to_string(),
                    bounds: Some((1.0, f64::INFINITY)),
                },
                VariableDefinitionInfo {
                    name: "num_objectives".to_string(),
                    description: "Number of objectives to optimize".to_string(),
                    variable_type: "integer".to_string(),
                    bounds: Some((1.0, f64::INFINITY)),
                },
                VariableDefinitionInfo {
                    name: "mutation_rate".to_string(),
                    description: "Probability of mutation for each variable".to_string(),
                    variable_type: "float".to_string(),
                    bounds: Some((0.0, 1.0)),
                },
                VariableDefinitionInfo {
                    name: "crossover_rate".to_string(),
                    description: "Probability of crossover for each pair of parents".to_string(),
                    variable_type: "float".to_string(),
                    bounds: Some((0.0, 1.0)),
                },
            ],
        }
    }

    /// Initialize random population
    fn initialize_population(&self) -> Vec<Solution> {
        let mut rng = rand::thread_rng();
        let mut population = Vec::with_capacity(self.population_size);

        for _ in 0..self.population_size {
            let variables: Vec<Variable> = self
                .variable_definitions
                .iter()
                .map(|def| match def {
                    VariableDefinition::Float(min, max) => {
                        Variable::Float(rng.gen_range(*min..*max))
                    }
                    VariableDefinition::Integer(min, max) => {
                        Variable::Integer(rng.gen_range(*min..=*max))
                    }
                    VariableDefinition::Boolean => Variable::Boolean(rng.gen_bool(0.5)),
                })
                .collect();
            population.push(Solution::new(variables, vec![0.0; self.num_objectives]));
        }

        population

        // // Parallel
        // (0..self.population_size)
        //     .into_par_iter()
        //     .map(|_| {
        //         let mut rng = rand::thread_rng();
        //         let variables: Vec<Variable> = self
        //             .variable_definitions
        //             .iter()
        //             .map(|def| match def {
        //                 VariableDefinition::Float(min, max) => {
        //                     Variable::Float(rng.gen_range(*min..*max))
        //                 }
        //                 VariableDefinition::Boolean => Variable::Boolean(rng.gen_bool(0.5)),
        //             })
        //             .collect();
        //         Solution::new(variables, vec![0.0; self.num_objectives])
        //     })
        //     .collect()
    }

    /// Perform non-dominated sorting to assign ranks
    fn non_dominated_sort(&self, population: &mut Vec<Solution>) -> Vec<Vec<usize>> {
        let mut fronts: Vec<Vec<usize>> = vec![Vec::new()];
        let mut domination_count: HashMap<usize, usize> = HashMap::new();
        let mut dominated_solutions: HashMap<usize, Vec<usize>> = HashMap::new();

        // Initialize the first front
        for i in 0..population.len() {
            domination_count.insert(i, 0);
            dominated_solutions.insert(i, Vec::new());

            for j in 0..population.len() {
                if i == j {
                    continue;
                }

                if self.dominates(&population[i], &population[j]) {
                    dominated_solutions.get_mut(&i).unwrap().push(j);
                } else if self.dominates(&population[j], &population[i]) {
                    *domination_count.get_mut(&i).unwrap() += 1;
                }
            }

            if domination_count[&i] == 0 {
                population[i].rank = 0;
                fronts[0].push(i);
            }
        }

        // Generate subsequent fronts
        let mut current_front = 0;
        while !fronts[current_front].is_empty() {
            let mut next_front = Vec::new();

            for &i in &fronts[current_front] {
                for &j in &dominated_solutions[&i] {
                    *domination_count.get_mut(&j).unwrap() -= 1;
                    if domination_count[&j] == 0 {
                        population[j].rank = current_front + 1;
                        next_front.push(j);
                    }
                }
            }

            current_front += 1;
            if !next_front.is_empty() {
                fronts.push(next_front);
            }

            if current_front >= fronts.len() {
                break;
            }
        }

        fronts
    }

    /// Calculate crowding distance for solutions in each front
    fn calculate_crowding_distance(&self, population: &mut Vec<Solution>, front: &[usize]) {
        let front_size = front.len();
        if front_size <= 2 {
            for &idx in front {
                population[idx].crowding_distance = f64::INFINITY;
            }
            return;
        }

        for &idx in front {
            population[idx].crowding_distance = 0.0;
        }

        for m in 0..self.num_objectives {
            let mut front_sorted: Vec<usize> = front.to_vec();
            front_sorted.sort_by(|a, b| {
                population[*a].objectives[m]
                    .partial_cmp(&population[*b].objectives[m])
                    .unwrap_or(Ordering::Equal)
            });

            // Set infinity for boundary points
            population[front_sorted[0]].crowding_distance = f64::INFINITY;
            population[front_sorted[front_size - 1]].crowding_distance = f64::INFINITY;

            // Calculate crowding distance for intermediate points
            let obj_min = population[front_sorted[0]].objectives[m];
            let obj_max = population[front_sorted[front_size - 1]].objectives[m];
            let scale = obj_max - obj_min;

            if scale > 0.0 {
                for i in 1..front_size - 1 {
                    population[front_sorted[i]].crowding_distance +=
                        (population[front_sorted[i + 1]].objectives[m]
                            - population[front_sorted[i - 1]].objectives[m])
                            / scale;
                }
            }
        }
    }

    /// Tournament selection
    fn tournament_selection(&self, population: &[Solution]) -> usize {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0..population.len());
        let b = rng.gen_range(0..population.len());

        if population[a].rank < population[b].rank {
            a
        } else if population[b].rank < population[a].rank {
            b
        } else if population[a].crowding_distance > population[b].crowding_distance {
            a
        } else {
            b
        }
    }

    /// Simulated Binary Crossover (SBX)
    fn crossover(&self, parent1: &Solution, parent2: &Solution) -> (Solution, Solution) {
        let mut rng = rand::thread_rng();
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        if rng.gen::<f64>() < self.crossover_rate {
            let eta_c = 20.0; // Distribution index for crossover

            for (i, def) in self.variable_definitions.iter().enumerate() {
                match def {
                    VariableDefinition::Float(min, max) => {
                        if rng.gen::<f64>() < 0.5 {
                            if let (Variable::Float(y1), Variable::Float(y2)) =
                                (&parent1.variables[i], &parent2.variables[i])
                            {
                                let beta = if y1 < y2 {
                                    1.0 + (2.0 * (y1 - min) / (y2 - y1))
                                } else {
                                    1.0 + (2.0 * (max - y1) / (y1 - y2))
                                };

                                let alpha = 2.0 - beta.powf(-eta_c - 1.0);
                                let rand = rng.gen::<f64>();
                                let betaq = if rand <= 1.0 / alpha {
                                    (rand * alpha).powf(1.0 / (eta_c + 1.0))
                                } else {
                                    (1.0 / (2.0 - rand * alpha)).powf(1.0 / (eta_c + 1.0))
                                };

                                let c1 = 0.5 * ((y1 + y2) - betaq * (y2 - y1));
                                let c2 = 0.5 * ((y1 + y2) + betaq * (y2 - y1));

                                child1.variables[i] = Variable::Float(c1.clamp(*min, *max));
                                child2.variables[i] = Variable::Float(c2.clamp(*min, *max));
                            }
                        }
                    }
                    VariableDefinition::Integer(min, max) => {
                        if rng.gen::<f64>() < 0.5 {
                            if let (Variable::Integer(y1), Variable::Integer(y2)) =
                                (&parent1.variables[i], &parent2.variables[i])
                            {
                                let y1_f = *y1 as f64;
                                let y2_f = *y2 as f64;
                                let min_f = *min as f64;
                                let max_f = *max as f64;

                                let beta = if y1_f < y2_f {
                                    1.0 + (2.0 * (y1_f - min_f) / (y2_f - y1_f))
                                } else {
                                    1.0 + (2.0 * (max_f - y1_f) / (y1_f - y2_f))
                                };

                                let alpha = 2.0 - beta.powf(-eta_c - 1.0);
                                let rand = rng.gen::<f64>();
                                let betaq = if rand <= 1.0 / alpha {
                                    (rand * alpha).powf(1.0 / (eta_c + 1.0))
                                } else {
                                    (1.0 / (2.0 - rand * alpha)).powf(1.0 / (eta_c + 1.0))
                                };

                                let c1 = 0.5 * ((y1_f + y2_f) - betaq * (y2_f - y1_f));
                                let c2 = 0.5 * ((y1_f + y2_f) + betaq * (y2_f - y1_f));

                                // Round to nearest integer and clamp to bounds
                                let c1_int = (c1.round() as i64).clamp(*min, *max);
                                let c2_int = (c2.round() as i64).clamp(*min, *max);

                                child1.variables[i] = Variable::Integer(c1_int);
                                child2.variables[i] = Variable::Integer(c2_int);
                            }
                        }
                    }
                    VariableDefinition::Boolean => {
                        // For boolean variables, randomly swap between parents
                        if rng.gen::<f64>() < 0.5 {
                            std::mem::swap(&mut child1.variables[i], &mut child2.variables[i]);
                        }
                    }
                }
            }
        }

        (child1, child2)
    }

    /// Modified mutation to handle mixed variables
    fn mutate(&self, solution: &mut Solution) {
        let mut rng = rand::thread_rng();
        let eta_m = 20.0; // Distribution index for mutation

        for (i, def) in self.variable_definitions.iter().enumerate() {
            if rng.gen::<f64>() < self.mutation_rate {
                match def {
                    VariableDefinition::Float(min, max) => {
                        if let Variable::Float(y) = solution.variables[i] {
                            let delta1 = (y - min) / (max - min);
                            let delta2 = (max - y) / (max - min);
                            let rnd = rng.gen::<f64>();
                            let deltaq;

                            if rnd <= 0.5 {
                                let xy = 1.0 - delta1;
                                let val = 2.0 * rnd + (1.0 - 2.0 * rnd) * xy.powf(eta_m + 1.0);
                                deltaq = val.powf(1.0 / (eta_m + 1.0)) - 1.0;
                            } else {
                                let xy = 1.0 - delta2;
                                let val =
                                    2.0 * (1.0 - rnd) + 2.0 * (rnd - 0.5) * xy.powf(eta_m + 1.0);
                                deltaq = 1.0 - val.powf(1.0 / (eta_m + 1.0));
                            }

                            let mutated = y + deltaq * (max - min);
                            solution.variables[i] = Variable::Float(mutated.clamp(*min, *max));
                        }
                    }
                    VariableDefinition::Integer(min, max) => {
                        if let Variable::Integer(y) = solution.variables[i] {
                            let y_f = y as f64;
                            let min_f = *min as f64;
                            let max_f = *max as f64;

                            let delta1 = (y_f - min_f) / (max_f - min_f);
                            let delta2 = (max_f - y_f) / (max_f - min_f);
                            let rnd = rng.gen::<f64>();
                            let deltaq;

                            if rnd <= 0.5 {
                                let xy = 1.0 - delta1;
                                let val = 2.0 * rnd + (1.0 - 2.0 * rnd) * xy.powf(eta_m + 1.0);
                                deltaq = val.powf(1.0 / (eta_m + 1.0)) - 1.0;
                            } else {
                                let xy = 1.0 - delta2;
                                let val =
                                    2.0 * (1.0 - rnd) + 2.0 * (rnd - 0.5) * xy.powf(eta_m + 1.0);
                                deltaq = 1.0 - val.powf(1.0 / (eta_m + 1.0));
                            }

                            let mutated_f = y_f + deltaq * (max_f - min_f);
                            // Round to nearest integer and clamp to bounds
                            let mutated = (mutated_f.round() as i64).clamp(*min, *max);
                            solution.variables[i] = Variable::Integer(mutated);
                        }
                    }
                    VariableDefinition::Boolean => {
                        if let Variable::Boolean(b) = solution.variables[i] {
                            solution.variables[i] = Variable::Boolean(!b);
                        }
                    }
                }
            }
        }
    }

    /// Check if solution a dominates solution b
    fn dominates(&self, a: &Solution, b: &Solution) -> bool {
        let mut one_is_better = false;

        for i in 0..self.num_objectives {
            if a.objectives[i] > b.objectives[i] {
                return false;
            }
            if a.objectives[i] < b.objectives[i] {
                one_is_better = true;
            }
        }

        one_is_better
    }

    // /// Parallelized offspring generation
    // fn generate_offspring_parallel(
    //     &self,
    //     population: &[Solution],
    //     evaluate: Arc<impl Fn(&[Variable]) -> Vec<f64> + Sync + Send>,
    // ) -> Vec<Solution> {
    //     (0..self.population_size)
    //         .into_par_iter()
    //         .map(|_| {
    //             let parent1_idx = self.tournament_selection(population);
    //             let parent2_idx = self.tournament_selection(population);
    //             let (mut child1, mut child2) =
    //                 self.crossover(&population[parent1_idx], &population[parent2_idx]);

    //             self.mutate(&mut child1);
    //             child1.objectives = evaluate(&child1.variables);
    //             child1
    //         })
    //         .collect()
    // }

    /// Run the NSGA-II algorithm
    pub fn run<F>(&self, generations: usize, evaluate: F) -> Vec<Solution>
    where
        F: Fn(&[Variable]) -> Vec<f64> + Clone + Sync + Send,
    {
        let mut population = self.initialize_population();

        // Evaluate initial population
        // for solution in &mut population {
        //     solution.objectives = evaluate(&solution.variables);
        // }

        // Parallel evaluation of initial population
        population.par_iter_mut().for_each(|solution| {
            solution.objectives = evaluate(&solution.variables);
        });

        for _ in 0..generations {
            // Create offspring population
            let mut offspring: Vec<Solution> = Vec::with_capacity(self.population_size);

            while offspring.len() < self.population_size {
                // Selection
                let parent1_idx = self.tournament_selection(&population);
                let parent2_idx = self.tournament_selection(&population);

                // Crossover
                let (mut child1, mut child2) =
                    self.crossover(&population[parent1_idx], &population[parent2_idx]);

                // Mutation
                self.mutate(&mut child1);
                self.mutate(&mut child2);

                offspring.push(child1);
                if offspring.len() < self.population_size {
                    offspring.push(child2);
                }
            }

            // Generate offspring in parallel
            // let offspring = self.generate_offspring_parallel(&population, evaluate.clone().into());

            // Parallel evaluation of offspring
            offspring.par_iter_mut().for_each(|child| {
                child.objectives = evaluate(&child.variables);
            });

            // Combine parent and offspring populations
            population.extend(offspring);

            // Non-dominated sorting
            let fronts = self.non_dominated_sort(&mut population);

            // Calculate crowding distance for each front
            for front in &fronts {
                self.calculate_crowding_distance(&mut population, front);
            }

            // Select next generation
            population.sort_by(|a, b| match a.rank.cmp(&b.rank) {
                Ordering::Equal => b
                    .crowding_distance
                    .partial_cmp(&a.crowding_distance)
                    .unwrap_or(Ordering::Equal),
                other => other,
            });

            population.truncate(self.population_size);
        }

        population
    }
}

// Implement the MetaheuristicTrait for NSGAII
impl MetaheuristicTrait for NSGAII {
    fn run(
        &self,
        num_generations: usize,
        evaluate: impl Fn(&[Variable]) -> Vec<f64> + Clone + Sync + Send,
    ) -> Vec<Solution> {
        self.run(num_generations, evaluate)
    }
}
