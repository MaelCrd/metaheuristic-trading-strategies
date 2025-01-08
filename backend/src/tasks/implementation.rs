use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::interface::handlers::{crypto_lists, crypto_symbols, mh_objects};
use crate::objects::klines::KlineCollection;
use crate::objects::objects::Task;

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

        let mh_object = mh_objects::get_mh_objects(&pool_state, Some(mh_object_id.to_string()))
            .await
            .into_inner();

        let mh_object = mh_object.first().unwrap();

        let crypto_list =
            crypto_lists::get_crypto_lists(&pool_state, Some(crypto_list_id.to_string()))
                .await
                .unwrap()
                .into_inner();

        let crypto_list = crypto_list.first().unwrap();

        let crypto_symbols = crypto_symbols::get_crypto_symbols(&pool_state)
            .await
            .into_inner()
            .iter()
            .filter(|&x| crypto_list.crypto_symbols.contains(&x.id))
            .cloned()
            .collect::<Vec<_>>();

        println!("[TASK {:?}] MHObject: {:?}", self.id, mh_object);
        println!("[TASK {:?}] CryptoList: {:?}", self.id, crypto_list);
        println!("[TASK {:?}] CryptoSymbols: {:?}", self.id, crypto_symbols);
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

        // Dummy task
        // let mut i: i64 = 0;
        // for _ in 0..i32::MAX {
        //     i += 1;
        // }

        Ok("Task completed successfully".to_string())
    }
}
