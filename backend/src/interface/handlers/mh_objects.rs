use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post, put};
use sqlx::{PgPool, Row};

use crate::objects::objects::{CreateMHObject, MHObject};

// Define a route to get all MHObjects
#[get("/mh_object?<id>")]
pub async fn get_mh_objects(pool: &State<PgPool>, id: Option<String>) -> Json<Vec<MHObject>> {
    let str_id = match id {
        Some(id) => format!("WHERE id = {}", id),
        None => "".to_string(),
    };
    let recs = sqlx::query(
        format!(
            r#"
            SELECT id, hidden, mh_algorithm_id, mh_parameters, other_parameters
            FROM mh_object {}
            "#,
            str_id
        )
        .as_str(),
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let mh_objects: Vec<MHObject> = recs
        .into_iter()
        .map(|row| MHObject {
            id: row.get("id"),
            hidden: row.get("hidden"),
            mh_algorithm_id: row.get("mh_algorithm_id"),
            mh_parameters: row.get("mh_parameters"),
            other_parameters: row.get("other_parameters"),
        })
        .collect();

    Json(mh_objects)
}

// Define a route to create a MHObject
#[post("/mh_object", format = "application/json", data = "<mh_object>")]
pub async fn create_mh_object(
    pool: &State<PgPool>,
    mh_object: Json<CreateMHObject>,
) -> Json<Vec<MHObject>> {
    // Insert the new MHObject
    sqlx::query!(
        r#"
        INSERT INTO mh_object (mh_algorithm_id, mh_parameters, other_parameters)
        VALUES ($1, $2, $3)
        "#,
        mh_object.mh_algorithm_id,
        mh_object.mh_parameters,
        mh_object.other_parameters,
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Return the MHObjects
    get_mh_objects(pool, None).await
}

// Define a route to hide/show a MHObject
#[put("/mh_object?<id>&<hidden>")]
pub async fn hide_mh_object(
    pool: &State<PgPool>,
    id: String,
    hidden: bool,
) -> Result<Json<Vec<MHObject>>, rocket::http::Status> {
    // Get the id of the MHObject to delete
    let id = id.parse::<i32>().unwrap();

    // Change the hidden status of the MHObject
    let res = sqlx::query!(
        r#"
        UPDATE mh_object
        SET hidden = $1
        WHERE id = $2
        "#,
        hidden,
        id,
    )
    .execute(&**pool)
    .await
    .unwrap();

    // Check if the MHObject was deleted
    if res.rows_affected() == 0 {
        println!("MHObject not found (id: {})", id);
        return Err(rocket::http::Status::NotFound);
    }

    // Return the MHObjects
    Ok(get_mh_objects(pool, None).await)
}
