use backend::{
    binance::klines,
    objects::{
        indicators::{Indicator, MovingAverage},
        objects::{CryptoInterval, KlineCollection},
    },
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
    if binance::klines::retrieve::retrieve_klines_simple(
        &mut klines_collection,
        "BTCUSDT",
        &CryptoInterval::Int1m,
        chrono::Duration::minutes(20).num_minutes(),
        0.75,
        false,
    )
    .await
    .is_err()
    {
        println!("Error retrieving klines");
        return;
    }

    klines_collection.display();
    klines_collection.check_integrity();

    // Print first and last open time
    println!(
        "First open time: {:?}",
        klines_collection.get_first_open_time()
    );
    println!(
        "Last open time: {:?}",
        klines_collection.get_last_open_time()
    );

    //
    let indicator_1 = Indicator::MovingAverage(MovingAverage {
        period: 4,
        values: Vec::<f64>::new(),
    });

    if binance::indicators::retrieve::retrieve_extended_klines(&mut klines_collection, &indicator_1)
        .await
        .is_err()
    {
        println!("Error retrieving klines");
        return;
    }

    klines_collection.display();
    klines_collection.check_integrity();

    // Indicator test
    let indicators: Vec<Indicator> = vec![
        Indicator::MovingAverage(MovingAverage {
            period: 3,
            values: Vec::<f64>::new(),
        }),
        Indicator::MovingAverage(MovingAverage {
            period: 7,
            values: Vec::<f64>::new(),
        }),
    ];

    if binance::indicators::retrieve::retrieve_extended_klines_max(
        &mut klines_collection,
        &indicators,
    )
    .await
    .is_err()
    {
        println!("Error retrieving extended klines");
        return;
    }

    klines_collection.display();
    klines_collection.check_integrity();

    return;

    // loading::test_print_loading();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    // rocket::rocket().launch().await.unwrap();
}
