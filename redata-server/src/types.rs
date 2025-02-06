use rocket::serde::uuid::Uuid;
use rocket_okapi::JsonSchema;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    /// A unique user identifier.
    pub blob_id: Uuid,
    /// Name of the blob
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunProperty {
    /// Property name
    pub name: String,

    /// Property value
    pub value: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    /// A unique user identifier.
    pub run_id: Uuid,
    /// The current username of the user.
    pub name: String,
    /// List of blobs associated with the run
    pub blobs: Vec<Blob>,
    /// Properties associated with the run
    pub properties: Vec<RunProperty>,
    /// List of tags associated with the run
    pub tags: Vec<String>,
}
