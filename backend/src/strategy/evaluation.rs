use crate::metaheuristic::{Metaheuristic, VariableDefinition, NSGAII};
use crate::objects::{indicators::Indicator, klines::KlineCollection, objects::MHObject};

// Evaluation of the strategy
pub fn evaluate(
    kline_collections: &Vec<KlineCollection>,
    indicators: &Vec<Indicator>,
    mh_object: &MHObject,
) -> Result<(), String> {
    println!("-> Evaluating the strategy");

    println!("-> Kline collections:");
    for kline_collection in kline_collections {
        kline_collection.display();
    }
    println!("-> Indicators: {:?}", indicators);

    // Evaluate the strategy
    let algorithm_name = mh_object.mh_algorithm_name.clone();
    let algorithm_parameters: serde_json::Value =
        serde_json::from_str(&mh_object.mh_parameters).unwrap();
    let population_size_option = algorithm_parameters.get("population_size");
    let mutation_rate_option = algorithm_parameters.get("mutation_rate");
    let crossover_rate_option = algorithm_parameters.get("crossover_rate");
    if population_size_option.is_none()
        || mutation_rate_option.is_none()
        || crossover_rate_option.is_none()
    {
        println!("-> Missing parameters for the algorithm");
        return Err("Missing parameters for the algorithm".to_string());
    }

    // Create variable definitions for the algorithm (variables to optimize, in this case, the indicators)
    let mut variable_definitions: Vec<VariableDefinition> = Vec::new();

    // Create algorithm
    let algorithm = match algorithm_name.as_str() {
        "NSGA-II" => Metaheuristic::NSGAII(NSGAII::new(
            population_size_option.unwrap().as_u64().unwrap() as usize,
            variable_definitions,
            3,
            mutation_rate_option.unwrap().as_f64().unwrap(),
            crossover_rate_option.unwrap().as_f64().unwrap(),
        )),
        _ => {
            println!("-> Unknown algorithm: {}", algorithm_name);
            return Err("Unknown algorithm".to_string());
        }
    };

    println!("-> Algorithm: {:?}", algorithm);

    Ok(())
}
