use serde_json::Number;

use crate::metaheuristic::{
    Metaheuristic, MetaheuristicTrait, MultiObjectiveDescent, Variable, VariableDefinition, NSGAII,
};
use crate::objects::indicators::IndicatorTrait;
use crate::objects::{
    criteria::Criterion, indicators::Indicator, klines::KlineCollection, objects::MHObject,
};

pub fn backtest(
    vars: &[Variable],
    kline_collections: &Vec<KlineCollection>,
    indicators: &Vec<Indicator>,
    variable_definitions_sep: &Vec<Vec<VariableDefinition>>,
) -> Vec<f64> {
    // let x = match vars[0] {
    //     Variable::Float(v) => v,
    //     _ => panic!("Expected float"),
    // };
    // let _b = match vars[1] {
    //     Variable::Boolean(v) => v,
    //     _ => panic!("Expected boolean"),
    // };
    // let y = match vars[2] {
    //     Variable::Integer(v) => v,
    //     _ => panic!("Expected integer"),
    // };

    // vec![
    //     x.powi(2),                // First objective
    //     (4.0 - (y as f64)),       // Second objective
    //     (x + (y as f64) / 100.0), // Third objective
    // ]

    println!("-> Backtesting with variables: {:?}", vars);

    let mut j = 0;
    for (i, indicator) in indicators.iter().enumerate() {
        let variable_definitions = &variable_definitions_sep[i];
        for variable_definition in variable_definitions {
            // Get the variable value
            let variable = &vars[j];
            let variable_str = match variable {
                Variable::Float(v) => v.to_string(),
                Variable::Integer(v) => v.to_string(),
                Variable::Boolean(v) => v.to_string(),
            };
            println!(
                "Indicator: {:?}, Variable: {:?}",
                indicator.information().name,
                variable_str
            );
            j += 1;
        }
    }

    vec![0.0, 0.0, 0.0]
}

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

    // Variables definitions for each indicator
    let variable_definitions_sep: Vec<Vec<VariableDefinition>> = indicators
        .iter()
        .map(|indicator| indicator.get_all_variable_definitions())
        .collect();

    // Create variable definitions for the algorithm (variables to optimize, in this case, the indicators)
    let variable_definitions: Vec<VariableDefinition> = variable_definitions_sep
        .iter()
        .flat_map(|v| v.clone())
        .collect();

    // Create algorithm
    let algorithm = match algorithm_name.as_str() {
        "NSGA-II" => {
            let algo = NSGAII::new_from_json(&algorithm_parameters, variable_definitions, 3);
            if algo.is_err() {
                return Err("Error creating NSGA-II algorithm".to_string());
            }
            Metaheuristic::NSGAII(algo.unwrap())
        }
        "Multi-Objective Simple Descent" => {
            let algo = MultiObjectiveDescent::new_from_json(
                &algorithm_parameters,
                variable_definitions,
                3,
            );
            if algo.is_err() {
                return Err("Error creating Multi-Objective Simple Descent algorithm".to_string());
            }
            Metaheuristic::MultiObjectiveDescent(algo.unwrap())
        }
        _ => {
            println!("-> Unknown algorithm: {}", algorithm_name);
            return Err("Unknown algorithm".to_string());
        }
    };

    println!("-> Algorithm: {:?}", algorithm);

    let final_solutions = algorithm.run(
        2,
        backtest,
        &kline_collections,
        &indicators,
        &variable_definitions_sep,
    );

    println!("-> Final solutions:");
    for solution in final_solutions {
        println!("{:?}", solution);
    }

    Ok(())
}
