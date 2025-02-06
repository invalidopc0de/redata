mod settings;
mod trino;
mod api;
mod types;
mod metadata_db_types;

#[macro_use] extern crate rocket;
use rocket_okapi::{openapi_get_routes, swagger_ui::*};
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use crate::settings::Settings;

#[derive(Database)]
#[database("db")]
pub struct Db(trino::TrinoPool);

#[launch]
fn rocket() -> _ {
    // Parse settings
    let settings = Settings::new();

    // Start API
    rocket::build()
        .manage(settings)
        .attach(Db::init())
        .mount("/api/v1", openapi_get_routes![
            api::get_run,
            api::create_run
        ])
        .mount("/api/v1/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount("/", FileServer::from("/www/static"))
}
