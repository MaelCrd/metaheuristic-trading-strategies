// use dotenv::dotenv;

mod listener; // Add this line to import the listener module
use backend::module;
use backend::utils;

use backend::interface::rocket;

#[tokio::main]
async fn main() {
    // assert!(dotenv().is_ok());

    // // Spawn the listener task
    // tokio::spawn(async move {
    //     listener::listen_for_updates(rx).await;
    // });

    // Run the function "helper_function" from the "utils" module
    utils::helpers::helper_function();

    // Run the function "function1" from the "submodule" module
    module::submodule::function1();

    // Wait 3 seconds before running the Rocket application
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Run the Rocket application
    rocket::rocket().launch().await.unwrap();
}
