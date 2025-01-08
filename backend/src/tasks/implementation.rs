use std::future::Future;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::interface::handlers::{crypto_lists, crypto_symbols, mh_objects};
use crate::objects::klines::KlineCollection;
use crate::objects::objects::{Task, TaskState};

impl Task {
    pub async fn execute(&self, should_cancel: Arc<AtomicBool>) -> Result<String, String> {
        println!("[TASK {:?}] Starting", self.id);

        let pool = crate::utils::db::get_new_pool().await;
        let pool_state = rocket::State::from(&pool);

        let mh_object_id = self.mh_object_id.unwrap();
        let crypto_list_id = self.crypto_list_id.unwrap();

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

        let all_crypto_symbols = crypto_symbols::get_crypto_symbols(&pool_state)
            .await
            .into_inner();

        let crypto_symbols = all_crypto_symbols
            .iter()
            .filter(|&x| crypto_list.crypto_symbols.contains(&x.id))
            .collect::<Vec<_>>();

        println!("[TASK {:?}] MHObject: {:?}", self.id, mh_object);
        println!("[TASK {:?}] CryptoList: {:?}", self.id, crypto_list);
        println!("[TASK {:?}] CryptoSymbols: {:?}", self.id, crypto_symbols);

        if should_cancel.load(Ordering::Relaxed) {
            return Err("Task was cancelled".to_string());
        }

        // Parameters
        let interval = &crypto_list.interval;
        // let limit_minutes = crypto_list.limit_minutes;
        // let training_percentage = crypto_list.training_percentage;
        // let force_fetch = crypto_list.force_fetch;
        let limit_minutes = 4000;
        let training_percentage = 0.8;
        let force_fetch = false;

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
                    crypto_symbol,
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
