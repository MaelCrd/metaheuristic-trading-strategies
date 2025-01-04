use std::env;

use backend::objects::{
    indicators::{Indicator, IndicatorTrait, MovingAverage, StochasticOscillator},
    intervals::CryptoInterval,
    klines::KlineCollection,
};
// use backend::objects::objects::CryptoSymbolSimple;
// use chrono::DateTime;
// use chrono::NaiveDateTime;
// use chrono::TimeZone;
// use chrono::Utc;
use dotenv::dotenv;

// mod listener; // Add this line to import the listener module

use backend::interface::rocket;
// use backend::utils::loading;
use backend::binance;
use backend::manager;
use backend::metaheuristic::mh;

#[tokio::main]
async fn main() {
    // Load the environment variables from the .env file
    assert!(dotenv().is_ok());

    // let args: Vec<String> = env::args().collect();
    // let minutes = args[1].parse::<i64>().unwrap();
    // let force_fetch = args[2].parse::<bool>().unwrap();

    //

    let task_manager = manager::TaskManager::new().await;
    tokio::spawn(async move {
        task_manager.start().await.unwrap();
    });

    // Run the Rocket application
    rocket::rocket().launch().await.unwrap();

    return;

    //

    //

    // mh::mh_nsga_ii();

    // return;

    //

    let minutes = 10 * 5;
    let force_fetch = false;

    let mut klines_collection: KlineCollection = KlineCollection::new();
    if binance::klines::retrieve::retrieve_klines_simple(
        &mut klines_collection,
        "BTCUSDT",
        &CryptoInterval::Int5m,
        chrono::Duration::minutes(minutes).num_minutes(),
        0.75,
        force_fetch,
    )
    .await
    .is_err()
    {
        println!("Error retrieving klines");
        return;
    }

    klines_collection.display();
    // klines_collection.check_integrity();

    // Print first and last open time
    println!(
        "First open time: {:?}",
        klines_collection.get_first_open_time()
    );
    println!(
        "Last open time: {:?}",
        klines_collection.get_last_open_time()
    );

    // let mut indicator_2: Indicator =
    //     Indicator::StochasticOscillator(StochasticOscillator::new(5, 3));
    let mut indicator_2 = Indicator::MovingAverage(MovingAverage::new(7));

    if binance::indicators::retrieve::retrieve_extended_klines(&mut klines_collection, &indicator_2)
        .await
        .is_err()
    {
        println!("Error retrieving klines");
        return;
    }

    klines_collection.display();

    // Indicator test
    if binance::indicators::retrieve::retrieve_indicator(&mut indicator_2, &klines_collection)
        .await
        .is_err()
    {
        println!("Error retrieving indicator");
        return;
    }

    println!("Indicator: {:?}", indicator_2);

    indicator_2.calculate_criteria(&klines_collection);
    let criteria = indicator_2.get_criteria();
    println!("Criteria: {:?}", criteria);
    for c in criteria {
        println!("Criteria: {:?}", c);
    }

    // println!("Final Indicator: {:?}", indicator_2);

    return;

    // loading::test_print_loading();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    // rocket::rocket().launch().await.unwrap();
}
