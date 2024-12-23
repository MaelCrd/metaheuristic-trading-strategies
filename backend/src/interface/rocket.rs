use chrono::{DateTime, NaiveDateTime, TimeZone};
// use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::fairing::AdHoc;
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
// use rocket::time::PrimitiveDateTime;
use chrono::Utc;
use rocket::time::OffsetDateTime;
use rocket::{get, post, routes};
use rocket::{http::Method, Build, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tokio::time::interval; // self,

use crate::binance::binance;
use crate::objects::objects::{
    CreateCryptoList, CryptoList, CryptoSymbol, CryptoSymbolSimple, Status,
};

#[get("/health")]
async fn health_check() -> Json<Status> {
    Json(Status {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// Define a route to create a new crypto list
#[post(
    "/crypto_list",
    format = "application/json",
    data = "<create_crypto_list>"
)]
async fn create_crypto_list(
    pool: &State<PgPool>,
    create_crypto_list: Json<CreateCryptoList>,
) -> Json<Vec<CryptoList>> {
    let _ = sqlx::query!(
        r#"
        INSERT INTO crypto_list (name, type)
        VALUES ($1, $2)
        RETURNING id, name, type
        "#,
        create_crypto_list.name,
        create_crypto_list.r#type,
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    get_crypto_lists(pool).await
}

// Define a route to get all crypto lists
#[get("/crypto_list")]
async fn get_crypto_lists(pool: &State<PgPool>) -> Json<Vec<CryptoList>> {
    let recs = sqlx::query!(
        r#"
        SELECT id, name, type
        FROM crypto_list
        "#,
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let crypto_lists: Vec<CryptoList> = recs
        .into_iter()
        .map(|row| CryptoList {
            id: row.id,
            name: row.name,
            r#type: row.r#type,
        })
        .collect();

    Json(crypto_lists)
}

/// Produce an infinite series of `"hello"`s, one per second.
#[get("/infinite-hellos")]
fn hello() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            yield "hello\n";
            interval.tick().await;
        }
    }
}

fn convert_primitive_to_chrono(primitive: PrimitiveDateTime) -> DateTime<Utc> {
    // To string
    let date_time_str = primitive.to_string();

    // Parse string to NaiveDateTime
    let naive_datetime = NaiveDateTime::parse_from_str(&date_time_str, "%Y-%m-%d %H:%M:%S%.f")
        .expect("Failed to parse datetime");

    // Convert NaiveDateTime to DateTime<Utc>
    Utc.from_utc_datetime(&naive_datetime).into()
}

// Define a route to reload the crypto symbols volume and availability
#[post("/crypto_symbol/reload")]
async fn reload_crypto_symbols(pool: &State<PgPool>) -> Json<Vec<CryptoSymbol>> {
    // Get all crypto symbols from the database
    let recs = sqlx::query!(
        r#"
        SELECT id, symbol, name, volume, last_updated
        FROM crypto_symbol
        "#,
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    // Convert the records to CryptoSymbol objects
    let crypto_symbols: Vec<CryptoSymbol> = recs
        .into_iter()
        .map(|row| CryptoSymbol {
            id: row.id,
            symbol: row.symbol,
            name: row.name,
            volume: row.volume,
            last_updated: convert_primitive_to_chrono(row.last_updated),
            available: true,
        })
        .collect();

    // Get the availability and volume of the crypto symbols
    println!("Getting actual info for all symbols");
    let mut new_symbols: Vec<CryptoSymbolSimple> = Vec::new();
    binance::get_symbols_actual_info(&mut new_symbols).await;

    // Print the 5 first new symbols
    for symbol in new_symbols.iter().take(5) {
        println!(
            "Symbol: {}, Volume: {}, Available: {}",
            symbol.symbol, symbol.volume, symbol.available
        );
    }

    // Get the current time
    let now_odt = OffsetDateTime::now_utc();
    let current_time: PrimitiveDateTime = PrimitiveDateTime::new(now_odt.date(), now_odt.time());

    // PART 1 : Update the crypto symbols present in the database
    println!("Updating crypto symbols in the database");
    let unknown_symbol: CryptoSymbolSimple = CryptoSymbolSimple {
        symbol: "".to_string(),
        volume: -1.0,
        available: false,
    };
    for symbol in crypto_symbols.iter() {
        // If the symbol is not in new_symbols, make it unavailable
        let new_symbol = new_symbols
            .iter()
            .find(|s| s.symbol == symbol.symbol)
            .unwrap_or_else(|| {
                println!("Symbol not found: {}", symbol.symbol);
                &unknown_symbol
            });

        // Update the symbol
        println!("Updating symbol: {:?}", new_symbol);
        sqlx::query!(
            r#"
            UPDATE crypto_symbol
            SET volume = $1, available = $2, last_updated = $3
            WHERE symbol = $4
            "#,
            new_symbol.volume,
            new_symbol.available,
            current_time,
            symbol.symbol,
        )
        .execute(&**pool)
        .await
        .unwrap();
    }

    // PART 2 : Add the new symbols to the database
    println!("Adding new symbols to the database");
    for symbol in new_symbols.iter() {
        // If the symbol is already in the database, skip
        if crypto_symbols.iter().any(|s| s.symbol == symbol.symbol) {
            continue;
        }
        // Insert the new symbol
        println!("Inserting new symbol: {}", symbol.symbol);
        sqlx::query!(
            r#"
            INSERT INTO crypto_symbol (symbol, name, volume, last_updated, available)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            symbol.symbol,
            "".to_string(),
            symbol.volume,
            current_time,
            symbol.available,
        )
        .execute(&**pool)
        .await
        .unwrap();
    }

    // Return the crypto symbols
    Json(get_crypto_symbols(pool).await.0)
}

// Define a route to get all crypto symbols
#[get("/crypto_symbol")]
async fn get_crypto_symbols(pool: &State<PgPool>) -> Json<Vec<CryptoSymbol>> {
    let recs = sqlx::query!(
        r#"
        SELECT id, symbol, name, volume, last_updated, available
        FROM crypto_symbol
        "#,
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let crypto_symbols: Vec<CryptoSymbol> = recs
        .into_iter()
        .map(|row| CryptoSymbol {
            id: row.id,
            symbol: row.symbol,
            name: row.name,
            volume: row.volume,
            last_updated: convert_primitive_to_chrono(row.last_updated),
            available: row.available.unwrap_or(false),
        })
        .collect();

    Json(crypto_symbols)
}

// ------------------------------------------------

// #[launch]
pub fn rocket() -> rocket::Rocket<Build> {
    // Configure CORS
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration error");

    // Check if the DATABASE_URL environment variable is set
    let _ = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Return the Rocket instance
    rocket::build()
        // .attach(Db::fairing())
        .configure(rocket::Config::figment().merge(("port", 9797)))
        .attach(AdHoc::on_ignite("Database", |rocket| async {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to create pool.");

            rocket.manage(pool)
        }))
        .mount(
            "/api",
            routes![
                health_check,
                create_crypto_list,
                get_crypto_lists,
                hello,
                reload_crypto_symbols,
                get_crypto_symbols,
            ],
        )
        .attach(cors)
}
