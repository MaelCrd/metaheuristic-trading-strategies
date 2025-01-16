use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::interface::handlers;
use crate::metaheuristic::mh;
use crate::objects::{indicators, klines::KlineCollection, objects::Task};

const FORCE_FETCH_DEFAULT: bool = false;
const TRAINING_PERCENTAGE_DEFAULT: f64 = 0.8;

impl Task {
    pub async fn execute(&self, should_cancel: Arc<AtomicBool>) -> Result<String, String> {
        println!("[TASK {:?}] Starting", self.id);

        let pool = crate::utils::db::get_new_pool().await;
        let pool_state = rocket::State::from(&pool);

        // Get MHObject and CryptoList ids
        let mh_object_id = self.mh_object_id.unwrap();
        let crypto_list_id = self.crypto_list_id.unwrap();
        // Convert other_parameters to serde_json::Value
        let other_parameters: serde_json::Value =
            serde_json::from_str(&self.other_parameters.clone().unwrap_or("{}".to_string()))
                .unwrap();

        // MH Object
        let mh_object =
            handlers::mh_objects::get_mh_objects(&pool_state, Some(mh_object_id.to_string()))
                .await
                .into_inner()
                .into_iter()
                .next()
                .unwrap();
        // let mh_object_parameters: serde_json::Value =
        //     serde_json::from_str(&mh_object.mh_parameters).unwrap();

        // Crypto List
        let crypto_list =
            handlers::crypto_lists::get_crypto_lists(&pool_state, Some(crypto_list_id.to_string()))
                .await
                .unwrap()
                .into_inner()
                .into_iter()
                .next()
                .unwrap();

        // Crypto Symbols
        let crypto_symbols = handlers::crypto_symbols::get_crypto_symbols(&pool_state)
            .await
            .into_inner()
            .iter()
            .filter(|&x| crypto_list.crypto_symbols.contains(&x.id))
            .cloned()
            .collect::<Vec<_>>();

        // Indicator combination & indicators
        let indicator_combination_id = self.indicator_combination_id.unwrap();
        let indicator_combination_option = handlers::indicators::get_indicator_combinations(
            &pool_state,
            Some(indicator_combination_id.to_string()),
        )
        .await
        .into_inner()
        .into_iter()
        .next();

        if indicator_combination_option.is_none() {
            return Err("Indicator combination not found".to_string());
        }
        let indicator_combination = indicator_combination_option.unwrap();

        let indicators_in_combination = handlers::indicators::get_indicators_in_combination(
            &pool_state,
            indicator_combination_id.to_string(),
        )
        .await
        .into_inner();

        // Indicators creation
        let mut indicators: Vec<indicators::Indicator> = vec![];
        for indicator_in_combination in indicators_in_combination {
            let parameters: serde_json::Value =
                serde_json::from_str(&indicator_in_combination.parameters.replace("'", "\""))
                    .unwrap();
            let indicator = indicators::Indicator::new_from_struct_name(
                &indicator_in_combination.indicator_struct_name,
                &parameters,
            );

            if let Some(indicator) = indicator {
                indicators.push(indicator);
            } else {
                println!(
                    "[TASK {:?}] Error creating indicator from struct name: {:?} (wrong struct name / parameters / ...)",
                    self.id, indicator_in_combination.indicator_struct_name
                );
                return Err("Error creating indicator".to_string());
            }
        }

        println!("[TASK {:?}] MHObject: {:?}", self.id, mh_object);
        println!("[TASK {:?}] CryptoList: {:?}", self.id, crypto_list);
        println!("[TASK {:?}] CryptoSymbols: {:?}", self.id, crypto_symbols);
        println!(
            "[TASK {:?}] IndicatorCombination: {:?}",
            self.id, indicator_combination
        );
        println!("[TASK {:?}] Indicators: {:?}", self.id, indicators);
        println!(
            "[TASK {:?}] OtherParameters: {:?}",
            self.id, other_parameters
        );

        if should_cancel.load(Ordering::Relaxed) {
            return Err("Task was cancelled".to_string());
        }

        // Parameters
        let interval = &crypto_list.interval;
        let limit_minutes = crypto_list.limit_count as i64 * interval.to_minutes();

        // Other parameters
        let force_fetch = match other_parameters.get("force_fetch") {
            Some(value) => value.as_bool().unwrap(),
            None => FORCE_FETCH_DEFAULT,
        };
        let training_percentage = match other_parameters.get("training_percentage") {
            Some(value) => value.as_f64().unwrap(),
            None => TRAINING_PERCENTAGE_DEFAULT,
        };

        // If limit_minutes is less than 10*interval, return err
        if limit_minutes < 10 * interval.to_minutes() {
            return Err("limit_minutes must be at least 10 times the interval".to_string());
        }

        // Kline Collections
        let mut kline_collections: Vec<KlineCollection> = vec![];
        kline_collections.reserve(crypto_symbols.len());
        for crypto_symbol in crypto_symbols {
            let mut kline_collection = KlineCollection::new();
            match kline_collection
                .retrieve_klines_simple(
                    &crypto_symbol,
                    interval,
                    limit_minutes,
                    training_percentage,
                    force_fetch,
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "[TASK {:?}] Error retrieving KlineCollection for {:?}: {:?}",
                        self.id, crypto_symbol, e
                    );
                }
            }

            kline_collections.push(kline_collection);
        }

        println!("[TASK {:?}] Indicators: {:?}", self.id, indicators);

        //

        // MHObject evaluation
        //
        println!(
            "[TASK {:?}] Evaluating MHObject (mh algorithm name : {})",
            self.id, mh_object.mh_algorithm_name
        );

        // Dummy task
        // let mut i: i64 = 0;
        // for _ in 0..i32::MAX {
        //     i += 1;
        // }

        Ok("Task completed successfully".to_string())
    }
}
