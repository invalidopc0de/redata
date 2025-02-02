#[macro_use] extern crate rocket;
use rocket::{get, post, serde::json::Json, serde::uuid::Uuid};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use rocket::fs::FileServer;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Blob {
    /// A unique user identifier.
    blob_id: Uuid,
    /// Name of the blob
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Property {
    /// Property name
    name: String,

    /// Property value
    value: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Run {
    /// A unique user identifier.
    run_id: Uuid,
    /// The current username of the user.
    name: String,
    /// List of blobs associated with the run
    blobs: Vec<Blob>,
    /// Properties associated with the run
    properties: Vec<Property>,
    /// List of tags associated with the run
    tags: Vec<String>,
}

/// # Get run
///
/// Returns a single run by ID
#[openapi(tag = "Runs")]
#[get("/run/<id>")]
fn get_run(id: Uuid) -> Option<Json<Run>> {
    Some(Json(Run {
        run_id: id,
        name: "Magic Test Data".to_owned(),
        blobs: vec![],
        properties: vec![],
        tags: vec![],
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", openapi_get_routes![
                get_run
            ],
        )
        .mount("/api/v1/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount("/", FileServer::from("/www/static"))
}
