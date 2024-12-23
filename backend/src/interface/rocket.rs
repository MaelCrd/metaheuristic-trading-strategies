use chrono::{DateTime, NaiveDateTime, TimeZone};
// use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::fairing::AdHoc;
use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
// use rocket::time::PrimitiveDateTime;
use chrono::Utc;
use rocket::time::OffsetDateTime;
use rocket::{get, post, put, routes};
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
    CreateCryptoList, CreateMHObject, CryptoListComplete, CryptoSymbol, CryptoSymbolSimple,
    MHObject, Status,
};

#[get("/health")]
async fn health_check() -> Json<Status> {
    Json(Status {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// Define a route to get all crypto lists
#[get("/crypto_list?<id>")]
async fn get_crypto_lists(
    pool: &State<PgPool>,
    id: Option<String>,
) -> Result<Json<Vec<CryptoListComplete>>, rocket::http::Status> {
    if let Some(id) = id {
        let id = id.parse::<i32>().unwrap();
        let recs = sqlx::query!(
            r#"
            SELECT id, hidden, name, type
            FROM crypto_list
            WHERE id = $1
            "#,
            id
        )
        .fetch_all(&**pool)
        .await
        .unwrap();

        // Check if the crypto list was found
        if recs.is_empty() {
            println!("Crypto list not found (id: {})", id);
            return Err(rocket::http::Status::NotFound);
        }

        // Get all the crypto symbols linked to the crypto list
        let crypto_symbols = sqlx::query!(
            r#"
            SELECT crypto_symbol_id
            FROM crypto_list_x_crypto_symbol
            WHERE crypto_list_id = $1
            "#,
            id
        )
        .fetch_all(&**pool)
        .await
        .unwrap();

        // Convert the records to CryptoListComplete objects
        let crypto_lists: Vec<CryptoListComplete> = recs
            .into_iter()
            .map(|row| CryptoListComplete {
                id: row.id,
                hidden: row.hidden,
                name: row.name,
                r#type: row.r#type,
                crypto_symbols: crypto_symbols
                    .iter()
                    .map(|row| row.crypto_symbol_id)
                    .collect(),
            })
            .collect();

        // Return the crypto list
        Ok(Json(crypto_lists))
    } else {
        let recs = sqlx::query!(
            r#"
            SELECT id, hidden, name, type
            FROM crypto_list
            "#,
        )
        .fetch_all(&**pool)
        .await
        .unwrap();

        let crypto_lists: Vec<CryptoListComplete> = recs
            .into_iter()
            .map(|row| CryptoListComplete {
                id: row.id,
                hidden: row.hidden,
                name: row.name,
                r#type: row.r#type,
                crypto_symbols: vec![],
            })
            .collect();

        Ok(Json(crypto_lists))
    }
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
) -> Result<Json<Vec<CryptoListComplete>>, rocket::http::Status> {
    // Insert the new crypto list and get its id
    let res = sqlx::query!(
        r#"
        INSERT INTO crypto_list (name, type)
        VALUES ($1, $2)
        RETURNING id
        "#,
        create_crypto_list.name,
        create_crypto_list.r#type,
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    // Get the id of the new crypto list
    let crypto_list_id = res.id;

    // Link the crypto symbols to the new crypto list using the crypto_list_x_crypto_symbol table
    for crypto_symbol_id in create_crypto_list.crypto_symbols.iter() {
        let result = sqlx::query!(
            r#"
            INSERT INTO crypto_list_x_crypto_symbol (crypto_list_id, crypto_symbol_id)
            VALUES ($1, $2)
            "#,
            crypto_list_id,
            crypto_symbol_id,
        )
        .execute(&**pool)
        .await;

        if let Err(e) = result {
            println!(
                "Failed to link crypto symbol to crypto list: {}\nReverting changes...",
                e
            );

            // Delete the crypto symbols linked to the crypto list
            let _ = sqlx::query!(
                r#"
                DELETE FROM crypto_list_x_crypto_symbol
                WHERE crypto_list_id = $1
                "#,
                crypto_list_id,
            )
            .execute(&**pool)
            .await
            .unwrap();

            // Delete the crypto list
            let _ = sqlx::query!(
                r#"
                DELETE FROM crypto_list
                WHERE id = $1
                "#,
                crypto_list_id,
            )
            .execute(&**pool)
            .await
            .unwrap();

            // Return an internal server error
            return Err(rocket::http::Status::InternalServerError);
        }
    }

    // Return the crypto lists
    Ok(get_crypto_lists(pool, None).await.unwrap())
}

// Define a route to hide/show a crypto list
#[put("/crypto_list?<id>&<hidden>")]
async fn hide_crypto_list(
    pool: &State<PgPool>,
    id: String,
    hidden: bool,
) -> Result<Json<Vec<CryptoListComplete>>, rocket::http::Status> {
    // Get the id of the crypto list to delete
    let id = id.parse::<i32>().unwrap();

    // Change the hidden status of the crypto list
    let res = sqlx::query!(
        r#"
        UPDATE crypto_list
        SET hidden = $1
        WHERE id = $2
        "#,
        hidden,
        id,
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Check if the crypto list was deleted
    if res.rows_affected() == 0 {
        println!("Crypto list not found (id: {})", id);
        return Err(rocket::http::Status::NotFound);
    }

    // Return the crypto lists
    Ok(get_crypto_lists(pool, None).await.unwrap())
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

// Define a route to get all MHObjects
#[get("/mh_object")]
async fn get_mh_objects(pool: &State<PgPool>) -> Json<Vec<MHObject>> {
    let recs = sqlx::query!(
        r#"
        SELECT id, hidden, mh_parameters, other_parameters
        FROM mh_object
        "#,
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let mh_objects: Vec<MHObject> = recs
        .into_iter()
        .map(|row| MHObject {
            id: row.id,
            hidden: row.hidden,
            mh_parameters: row.mh_parameters,
            other_parameters: row.other_parameters,
        })
        .collect();

    Json(mh_objects)
}

// Define a route to create a MHObject
#[post("/mh_object", format = "application/json", data = "<mh_object>")]
async fn create_mh_object(
    pool: &State<PgPool>,
    mh_object: Json<CreateMHObject>,
) -> Json<Vec<MHObject>> {
    // Insert the new MHObject
    sqlx::query!(
        r#"
        INSERT INTO mh_object (mh_parameters, other_parameters)
        VALUES ($1, $2)
        "#,
        mh_object.mh_parameters,
        mh_object.other_parameters,
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Return the MHObjects
    get_mh_objects(pool).await
}

// // Define a route to hide/show a MHObject
// #[put("/mh_object?<id>&<hidden>")]

// // Define a route to get all tasks
// #[get("/task")]
// async fn get_tasks(pool: &State<PgPool>) -> Json<Vec<Task>> {
//     let recs = sqlx::query!(
//         r#"
//         SELECT id, state, other_parameters
//         FROM task
//         "#,
//     )
//     .fetch_all(&**pool)
//     .await
//     .unwrap();

//     let tasks: Vec<Task> = recs
//         .into_iter()
//         .map(|row| Task {
//             id: row.id,
//             state: row.state,
//             other_parameters: row.other_parameters,
//             mh_object_id: None,
//             crypto_list_id: None,
//             result_id: None,
//         })
//         .collect();

//     Json(tasks)
// }

// ------------------------------------------------

// Define the Rocket instance
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
                hide_crypto_list,
                hello,
                reload_crypto_symbols,
                get_crypto_symbols,
                get_mh_objects,
                create_mh_object
            ],
        )
        .attach(cors)
}
