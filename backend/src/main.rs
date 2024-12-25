use backend::objects::{
    indicators::{Indicator, MovingAverage},
    objects::{CryptoInterval, Kline, KlineCollection},
};
// use backend::objects::objects::CryptoSymbolSimple;
// use chrono::DateTime;
// use chrono::NaiveDateTime;
// use chrono::TimeZone;
// use chrono::Utc;
use dotenv::dotenv;

// mod listener; // Add this line to import the listener module

// use backend::interface::rocket;
// use backend::utils::loading;
use backend::binance;

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
    let result = binance::klines::retrieve::retrieve_klines_simple(
        &mut klines_collection,
        "BTCUSDT",
        &CryptoInterval::Int1m,
        chrono::Duration::minutes(20).num_minutes(),
        0.75,
        false,
    )
    .await;

    if result.is_err() {
        println!("Error: {:?}", result.err().unwrap());
        return;
    }

    println!(
        "Klines collection: {:?} / {:?}",
        klines_collection.symbol, klines_collection.interval
    );

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

    // Print first and last open time
    println!(
        "First open time: {:?}",
        klines_collection.get_first_open_time()
    );
    println!(
        "Last open time: {:?}",
        klines_collection.get_last_open_time()
    );

    // println!("Klines training:");
    // for kline in klines_collection.training.iter() {
    //     print!("{:?} / ", kline.open_time);
    // }
    // println!();
    // println!("Klines validation:");
    // for kline in klines_collection.validation.iter() {
    //     print!("{:?} / ", kline.open_time);
    // }
    // println!();

    // return;

    // Indicator test
    let mut indicator: Indicator = Indicator::MovingAverage(MovingAverage {
        period: 3,
        values: Vec::<f64>::new(),
    });
    let result =
        binance::indicators::retrieve::retrieve_indicator(&klines_collection, &mut indicator).await;

    if result.is_err() {
        println!("Error: {:?}", result.err().unwrap());
        return;
    }

    return;

    // loading::test_print_loading();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    // rocket::rocket().launch().await.unwrap();
}
