use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, put};
use sqlx::{PgPool, Row};

use crate::objects::objects::MHAlgorithm;

// Define a route to get all MHAlgorithms
#[get("/mh_algorithms?<id>")]
pub async fn get_mh_algorithms(pool: &State<PgPool>, id: Option<String>) -> Json<Vec<MHAlgorithm>> {
    let str_id = match id {
        Some(id) => format!("WHERE id = {}", id),
        None => "".to_string(),
    };
    let recs = sqlx::query(
        format!(
            r#"
            SELECT id, name, version, hidden, parameters
            FROM mh_algorithm {}
            "#,
            str_id
        )
        .as_str(),
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let mh_algorithms: Vec<MHAlgorithm> = recs
        .into_iter()
        .map(|row| MHAlgorithm {
            id: row.get("id"),
            name: row.get("name"),
            version: row.get("version"),
            hidden: row.get("hidden"),
            parameters: row.get("parameters"),
        })
        .collect();

    Json(mh_algorithms)
}

// Define a route to hide/show a MHAlgorithm
#[put("/mh_algorithms?<id>&<hidden>")]
pub async fn hide_mh_algorithm(
    pool: &State<PgPool>,
    id: String,
    hidden: bool,
) -> Result<Json<Vec<MHAlgorithm>>, rocket::http::Status> {
    // Get the id of the MHAlgorithm to delete
    let id = id.parse::<i32>().unwrap();

    // Change the hidden status of the MHAlgorithm
    let res = sqlx::query!(
        r#"
        UPDATE mh_algorithm
        SET hidden = $1
        WHERE id = $2
        "#,
        hidden,
        id,
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Check if the MHAlgorithm was deleted
    if res.rows_affected() == 0 {
        println!("MHAlgorithm not found (id: {})", id);
        return Err(rocket::http::Status::NotFound);
    }

    // Return the MHAlgorithms
    Ok(get_mh_algorithms(pool, None).await)
}
