// use std::env;

// use backend::objects::{
//     self,
//     indicators::{Indicator, IndicatorTrait, MovingAverage, StochasticOscillator},
//     intervals::CryptoInterval,
//     klines::KlineCollection,
//     objects::CryptoSymbol,
// };
// use backend::objects::objects::CryptoSymbolSimple;
// use chrono::DateTime;
// use chrono::NaiveDateTime;
// use chrono::TimeZone;
// use chrono::Utc;
use dotenv::dotenv;

// mod listener; // Add this line to import the listener module

use backend::interface::rocket;
// use backend::utils::loading;
// use backend::binance;
use backend::manager;
use backend::metaheuristic::mh;

use backend::interface::handlers::streams;

#[tokio::main]
async fn main() {
    // Load the environment variables from the .env file
    assert!(dotenv().is_ok());

    // Create the Rocket application
    let rocket_app = rocket::rocket();

    // Create the TaskManager
    let task_manager = manager::TaskManager::new(streams::TaskStateChannel {
        sender: rocket_app
            .state::<streams::TaskStateChannel>()
            .unwrap()
            .sender
            .clone(),
    })
    .await;

    // Start the TaskManager
    tokio::spawn(async move {
        task_manager.start().await.unwrap();
    });

    // Run the Rocket application
    rocket_app.launch().await.unwrap();

    return;

    //

    //

    // mh::mh_nsga_ii();
    // mh::mh_descent();

    // return;

    //

    // let minutes = 10 * 5;
    // let force_fetch = false;

    // let crypto_symbol = CryptoSymbol {
    //     id: 1,
    //     symbol: "BTCUSDT".to_string(),
    //     name: "Bitcoin".to_string(),
    //     volume: 100.0,
    //     last_updated: chrono::Utc::now(),
    //     available: true,
    // };
    // let mut klines_collection: KlineCollection = KlineCollection::new();
    // if klines_collection
    //     .retrieve_klines_simple(
    //         &crypto_symbol,
    //         &CryptoInterval::Int5m,
    //         chrono::Duration::minutes(minutes).num_minutes(),
    //         0.75,
    //         force_fetch,
    //     )
    //     .await
    //     .is_err()
    // {
    //     println!("Error retrieving klines");
    //     return;
    // }

    // klines_collection.display();
    // // klines_collection.check_integrity();

    // // Print first and last open time
    // println!(
    //     "First open time: {:?}",
    //     klines_collection.get_first_open_time()
    // );
    // println!(
    //     "Last open time: {:?}",
    //     klines_collection.get_last_open_time()
    // );

    // // let mut indicator_2: Indicator =
    // //     Indicator::StochasticOscillator(StochasticOscillator::new(5, 3));
    // let mut indicator_2 = Indicator::MovingAverage(MovingAverage::new(7));

    // if klines_collection
    //     .retrieve_extended_klines(&indicator_2)
    //     .await
    //     .is_err()
    // {
    //     println!("Error retrieving klines");
    //     return;
    // }

    // klines_collection.display();

    // // Indicator test
    // if indicator_2.retrieve(&klines_collection).await.is_err() {
    //     println!("Error retrieving indicator");
    //     return;
    // }

    // println!("Indicator: {:?}", indicator_2);

    // indicator_2.calculate_criteria(&klines_collection);
    // let criteria = indicator_2.get_criteria();
    // println!("Criteria: {:?}", criteria);
    // for c in criteria {
    //     println!("Criteria: {:?}", c);
    // }

    // // println!("Final Indicator: {:?}", indicator_2);

    // return;

    // loading::test_print_loading();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    // rocket::rocket().launch().await.unwrap();
}
