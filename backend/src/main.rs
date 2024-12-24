use backend::objects::objects::{CryptoInterval, Kline, KlineCollection};
// use backend::objects::objects::CryptoSymbolSimple;
// use chrono::DateTime;
// use chrono::NaiveDateTime;
// use chrono::TimeZone;
// use chrono::Utc;
use dotenv::dotenv;

// mod listener; // Add this line to import the listener module

use backend::interface::rocket;

use backend::binance;

use backend::utils::loading;

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

    let mut klines_collection: KlineCollection = KlineCollection::new();
    let result = binance::klines::retrieve::retrieve_klines(
        &mut klines_collection,
        "ETHUSDT",
        &CryptoInterval::Int12h,
        chrono::Duration::days(17).num_minutes(),
        0.8,
    )
    .await;

    if result.is_err() {
        println!("Error: {:?}", result.err().unwrap());
        return;
    }

    println!(
        "Klines length (train): {}",
        klines_collection.training.len()
    );
    println!(
        "Klines length (validation): {}",
        klines_collection.validation.len()
    );
    println!(
        "Klines length (total): {}",
        klines_collection.training.len() + klines_collection.validation.len()
    );
    println!("Klines training:");
    for kline in klines_collection.training.iter() {
        println!("{:?}", kline.open_time);
    }
    println!("Klines validation:");
    for kline in klines_collection.validation.iter() {
        println!("{:?}", kline.open_time);
    }

    return;

    loading::test_print_loading();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    rocket::rocket().launch().await.unwrap();
}
