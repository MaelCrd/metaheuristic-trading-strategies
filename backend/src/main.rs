use std::env;

use backend::objects::objects::{CryptoInterval, Kline};
// use backend::objects::objects::CryptoSymbolSimple;
// use chrono::DateTime;
// use chrono::NaiveDateTime;
// use chrono::TimeZone;
// use chrono::Utc;
use dotenv::dotenv;

// mod listener; // Add this line to import the listener module

use backend::interface::rocket;

use backend::{binance, utils};

use backend::utils::loading;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Load the environment variables from the .env file
    assert!(dotenv().is_ok());

    // // Spawn the listener task
    // tokio::spawn(async move {
    //     listener::listen_for_updates(rx).await;
    // });

    // Call the function "print_all_symbols"
    // let mut symbol_volumes_vec: Vec<CryptoSymbolSimple> = Vec::new();
    // binance::get_symbols_actual_info(&mut symbol_volumes_vec).await;

    let mut klines: Vec<Kline> = Vec::new();
    let result = binance::klines::retrieve::retrieve_klines(
        &mut klines,
        "ETHUSDT",
        &CryptoInterval::Int12h,
        utils::time::days_to_minutes(20),
    )
    .await;

    if result.is_err() {
        println!("Error: {:?}", result.err().unwrap());
        return;
    }

    println!("Klines length: {}", klines.len());
    for kline in klines {
        println!("{:?}", kline.open_time);
    }

    return;

    loading::test_print_loading();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    rocket::rocket().launch().await.unwrap();
}
