use rocket::serde::json::Json;
use rocket::time::OffsetDateTime;
use rocket::State;
use rocket::{get, post};
use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;

use crate::binance::binance;
use crate::objects::objects::{CryptoSymbol, CryptoSymbolSimple};
use crate::utils;

// Define a route to reload the crypto symbols volume and availability
#[post("/crypto_symbol/reload")]
pub async fn reload_crypto_symbols(pool: &State<PgPool>) -> Json<Vec<CryptoSymbol>> {
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
            last_updated: utils::datetime::convert_primitive_to_chrono(row.last_updated),
            available: true,
        })
        .collect();

    // Get the availability and volume of the crypto symbols
    println!("Getting actual info for all symbols");
    let mut new_symbols: Vec<CryptoSymbolSimple> = Vec::new();
    binance::get_symbols_actual_info(&mut new_symbols).await;

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
pub async fn get_crypto_symbols(pool: &State<PgPool>) -> Json<Vec<CryptoSymbol>> {
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
            last_updated: utils::datetime::convert_primitive_to_chrono(row.last_updated),
            available: row.available.unwrap_or(false),
        })
        .collect();

    Json(crypto_symbols)
}
