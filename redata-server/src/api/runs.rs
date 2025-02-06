use rocket::{get, post, serde::json::Json, serde::uuid::Uuid};
use rocket_okapi::{openapi};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use rocket_db_pools::{Connection};
use sea_query::{PostgresQueryBuilder, Query, QueryStatementWriter};

use crate::types::Run;
use crate::metadata_db_types::*;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct CreateRunData {
    /// The current username of the user.
    name:  String,
}

/// # Create run
///
/// Returns a single run by ID
#[openapi(tag = "Runs")]
#[post("/run", data = "<data>")]
pub async fn create_run(db: Connection<crate::Db>, data: Json<CreateRunData>) -> Option<Json<Run>> {
    // Create new run query
    let insert_run_query = Query::insert()
        .into_table(MetadataRun::Table)
        .columns([MetadataRun::Name])
        .values_panic([data.name.to_string().into()])
        .to_owned()
        .to_string(PostgresQueryBuilder);

    println!("insert query: {:?}", insert_run_query);

    // Execute query
    db.execute(insert_run_query).await.unwrap();

    // Return newly create run info
    let new_run = Run {
        run_id: "3fa85f64-5717-4562-b3fc-2c963f66afa6".parse().unwrap(),
        name: data.name.to_string(),
        blobs: vec![],
        properties: vec![],
        tags: vec![],
    };

    Some(Json(new_run))
}

/// # Get run
///
/// Returns a single run by ID
#[openapi(tag = "Runs")]
#[get("/run/<id>")]
pub fn get_run(db: Connection<crate::Db>, id: Uuid) -> Option<Json<Run>> {
    Some(Json(Run {
        run_id: id,
        name: "Magic Test Data".to_owned(),
        blobs: vec![],
        properties: vec![],
        tags: vec![],
    }))
}
