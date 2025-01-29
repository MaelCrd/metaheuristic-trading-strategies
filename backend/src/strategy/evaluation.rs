use serde_json::Number;

use crate::metaheuristic::{Metaheuristic, MultiObjectiveDescent, VariableDefinition, NSGAII};
use crate::objects::indicators::IndicatorTrait;
use crate::objects::{
    criteria::Criterion, indicators::Indicator, klines::KlineCollection, objects::MHObject,
};

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

    println!("Parameters: {:?}", algorithm_parameters);

    // Create variable definitions for the algorithm (variables to optimize, in this case, the indicators)
    let mut variable_definitions: Vec<VariableDefinition> = Vec::new();
    for indicator in indicators {
        let parameters = indicator.information().parameters;
        for param in parameters {
            println!("Indicator parameter: {:?}", param.r#type);
        }
    }

    // Create algorithm
    let algorithm = match algorithm_name.as_str() {
        "NSGA-II" => {
            let algo = NSGAII::new_from_json(&algorithm_parameters, variable_definitions, 3);
            if algo.is_err() {
                return Err("Error creating NSGA-II algorithm".to_string());
            }
            Metaheuristic::NSGAII(algo.unwrap())
        }
        _ => {
            println!("-> Unknown algorithm: {}", algorithm_name);
            return Err("Unknown algorithm".to_string());
        }
    };

    println!("-> Algorithm: {:?}", algorithm);

    Ok(())
}
