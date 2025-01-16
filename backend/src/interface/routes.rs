use super::handlers::*;
use rocket::{routes, Route};

pub fn get_routes() -> Vec<Route> {
    routes![
        // General
        general::hello,
        general::health_check,
        general::purge_hidden_orphans,
        // Crypto Symbols
        crypto_symbols::get_crypto_symbols,
        crypto_symbols::reload_crypto_symbols,
        // Crypto Lists
        crypto_lists::get_crypto_lists,
        crypto_lists::create_crypto_list,
        crypto_lists::hide_crypto_list,
        // MH Objects
        mh_objects::get_mh_objects,
        mh_objects::create_mh_object,
        mh_objects::hide_mh_object,
        // MH Algorithms
        mh_algorithms::get_algorithms,
        // Tasks
        tasks::get_tasks,
        tasks::create_task,
        tasks::queue_task,
        tasks::cancel_task,
        // Indicators
        indicators::get_indicators,
        indicators::get_indicators_in_combination,
        indicators::get_indicator_combinations,
        indicators::create_indicator_combination,
    ]
}
