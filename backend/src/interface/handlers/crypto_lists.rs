use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post, put};
use sqlx::PgPool;

use crate::objects::objects::{CreateCryptoList, CryptoInterval, CryptoListComplete};

// Define a route to get all crypto lists
#[get("/crypto_list?<id>")]
pub async fn get_crypto_lists(
    pool: &State<PgPool>,
    id: Option<String>,
) -> Result<Json<Vec<CryptoListComplete>>, rocket::http::Status> {
    if let Some(id) = id {
        let id = id.parse::<i32>().unwrap();
        let recs = sqlx::query!(
            r#"
            SELECT id, hidden, name, interval::TEXT as interval, type
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
                interval: CryptoInterval::parse_from(row.interval.as_deref().unwrap()),
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
            SELECT id, hidden, name, interval::TEXT as interval, type
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
                interval: CryptoInterval::parse_from(row.interval.as_deref().unwrap()),
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
pub async fn create_crypto_list(
    pool: &State<PgPool>,
    create_crypto_list: Json<CreateCryptoList>,
) -> Result<Json<Vec<CryptoListComplete>>, rocket::http::Status> {
    // Insert the new crypto list and get its id
    let res = sqlx::query!(
        r#"
        INSERT INTO crypto_list (name, interval, type)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        create_crypto_list.name,
        create_crypto_list.interval.clone() as CryptoInterval,
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
pub async fn hide_crypto_list(
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
