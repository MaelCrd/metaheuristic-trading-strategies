use std::future::Future;
use std::sync::{atomic::AtomicBool, Arc};

use crate::interface::handlers::{crypto_lists, crypto_symbols, mh_objects};
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

        let mut i: i64 = 0;
        for _ in 0..i32::MAX {
            i += 1;
        }

        Ok("Task completed successfully".to_string())
    }
}
