use binance_spot_connector_rust::{
    http::{request::RequestBuilder, Method},
    hyper::BinanceHttpClient,
};

use crate::objects::objects::CryptoSymbolSimple;

const BINANCE_FUTURES_API_URL: &str = "https://fapi.binance.com";

pub async fn get_symbols_actual_info(symbol_volumes_vec: &mut Vec<CryptoSymbolSimple>) {
    // Create a new BinanceHttpClient
    let futures_client = BinanceHttpClient::with_url(BINANCE_FUTURES_API_URL);

    // Create a new custom endpoint '/fapi/v1/exchangeInfo' to fetch the exchange info
    let builder_exchange_info = RequestBuilder::new(Method::Get, "/fapi/v1/exchangeInfo");

    let data = futures_client
        .send(builder_exchange_info)
        .await
        .expect("Failed to get exchange info")
        .into_body_str()
        .await
        .expect("Failed to convert response to string");

    let data: serde_json::Value =
        serde_json::from_str(&data).expect("Failed to parse exchange info");

    let symbols = data["symbols"].as_array().expect("Failed to get symbols");

    let mut symbols_availability: Vec<(String, bool)> = Vec::new();

    for symbol in symbols {
        let symbol_info = symbol.as_object().expect("Failed to get symbol");
        let symbol = symbol_info["symbol"]
            .as_str()
            .expect("Failed to get symbol");
        let status = symbol_info["status"]
            .as_str()
            .expect("Failed to get status");

        let symbol = symbol.to_string();
        symbols_availability.push((symbol, status == "TRADING"));
    }

    //

    //

    // Create a new custom endpoint '/fapi/v1/ticker/24hr' to fetch the 24h ticker
    let builder_ticker_24h = RequestBuilder::new(Method::Get, "/fapi/v1/ticker/24hr");

    // Send the request to the endpoint
    let data = futures_client
        .send(builder_ticker_24h)
        .await
        .expect("Failed to get 24h ticker")
        .into_body_str()
        .await
        .expect("Failed to convert response to string");

    // Parse the response
    let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to parse 24h ticker");

    // Get the data as an array
    let data = data.as_array().expect("Failed to get 24h ticker");

    // For each symbol in the data, get the symbol and calculate the volume
    // Add the symbol and its volume to the Vec
    for symbol in data {
        // Print the symbol
        // println!("{:?}", symbol);

        // Get the symbol and volume
        let symbol_info = symbol.as_object().expect("Failed to get symbol");
        let symbol = symbol_info["symbol"]
            .as_str()
            .expect("Failed to get symbol");
        let volume = symbol_info["volume"]
            .as_str()
            .expect("Failed to get volume");

        // Multiply by weightedAvgPrice to get the volume in USD
        let volume = volume.parse::<f64>().expect("Failed to parse volume")
            * symbol_info["weightedAvgPrice"]
                .as_str()
                .expect("Failed to get weighted average price")
                .parse::<f64>()
                .expect("Failed to parse weighted average price") as f64;

        // Get the availability of the symbol
        let available = symbols_availability
            .iter()
            .find(|(s, _)| s == symbol)
            .expect("Failed to get availability")
            .1;

        // Add the symbol and its volume to the Vec
        symbol_volumes_vec.push(CryptoSymbolSimple {
            symbol: symbol.to_string(),
            volume,
            available,
        });
    }
}

//

//

// pub async fn associate_all_market_cap() -> Result<i32, Error> {
//     // Send a request to 'api.coincap.io/v2/assets?limit=2000'
//     // curl --location 'api.coincap.io/v2/assets?limit=1'
//     // {"data":[{"id":"bitcoin","rank":"1","symbol":"BTC","name":"Bitcoin","supply":"19794440.0000000000000000","maxSupply":"21000000.0000000000000000","marketCapUsd":"2002831506052.0666212442396120","volumeUsd24Hr":"20531439466.3153735396453840","priceUsd":"101181.5189544168272123","changePercent24Hr":"5.5501760625331975","vwap24Hr":"98345.8994129662141082","explorer":"https://blockchain.info/"}],"timestamp":1733946921767}
//     let client = reqwest::Client::new();

//     let response = client
//         .get("https://api.coincap.io/v2/assets?limit=2000")
//         .send()
//         .await
//         .expect("Failed to send request")
//         .json::<serde_json::Value>()
//         .await
//         .expect("Failed to parse response");

//     let data = response["data"].as_array().expect("Failed to get data");

//     let mut market_cap_map: Map<String, f64> = BTreeMap::new();

//     let mut count = 0;
//     for asset in data {
//         count += 1;

//         let asset = asset.as_object().expect("Failed to get asset");

//         let symbol = asset["symbol"].as_str().expect("Failed to get symbol");
//         if let Some(market_cap_str) = asset["marketCapUsd"].as_str() {
//             let result: f64;
//             if let Ok(market_cap) = market_cap_str.parse::<f64>() {
//                 // Use market_cap here
//                 print!("{symbol} : {market_cap} / ");
//                 result = market_cap;
//             } else {
//                 // Handle parse error, continue to next
//                 result = 0.0;
//             }
//             market_cap_map.insert(symbol.to_string(), result);
//         }
//     }
//     println!("");

//     let symbols = get_all_symbols().await.unwrap();

//     // For each symbol in the list of symbols
//     // print the symbol and the market cap
//     for symbol in symbols {
//         if let Some(market_cap) = market_cap_map.get(&symbol) {
//             println!("{symbol} : {market_cap}");
//         } else {
//             println!("{symbol} : UNKNOWN");
//         }
//     }

//     Ok(count)
// }
