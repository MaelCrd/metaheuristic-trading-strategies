use super::nsga2::NSGAII;
use super::objects::{Metaheuristic, MetaheuristicTrait, Variable, VariableDefinition};

pub fn mh_nsga_ii() {
    // Define your problem with mixed variables
    let population_size = 100;
    let variable_definitions = vec![
        VariableDefinition::Float(-1.0, 1.0), // Float variable
        VariableDefinition::Boolean,          // Boolean variable
        VariableDefinition::Float(0.0, 5.0),  // Another float variable
    ];
    let num_objectives = 3;
    let mutation_rate = 0.1;
    let crossover_rate = 0.9;

    // Create NSGA-II instance
    let nsga2 = Metaheuristic::NSGAII(NSGAII::new(
        population_size,
        variable_definitions,
        num_objectives,
        mutation_rate,
        crossover_rate,
    ));

    // Define your objective functions
    let evaluate = |vars: &[Variable]| -> Vec<f64> {
        let x = match vars[0] {
            Variable::Float(v) => v,
            _ => panic!("Expected float"),
        };
        let b = match vars[1] {
            Variable::Boolean(v) => v,
            _ => panic!("Expected boolean"),
        };
        let y = match vars[2] {
            Variable::Float(v) => v,
            _ => panic!("Expected float"),
        };

        vec![
            if b { x.powi(4) } else { x.powi(2) } + y / 10.0, // First objective
            (y - 2.0).powi(2),                                // Second objective
            (x + y) / (if b { 1.0 } else { 5.0 }),            // Third objective
        ]
    };

    // Now time
    let start = std::time::Instant::now();

    // Run optimization
    let final_population = nsga2.run(200, evaluate);
    let elapsed = start.elapsed();

    // Print results
    println!("Elapsed time: {:?}", elapsed);

    println!("Final population ({}):", final_population.len());
    for individual in final_population {
        println!("{}", individual.show_short());
    }
}
