#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;

use std::fs;

use parking_lot::Mutex;
use rocket_okapi::{
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use simple_logger::SimpleLogger;

use crate::{
    database::{memory::InMemoryDatabase, DatabaseHolder},
    representation::config::{trustifier_config, GeneralConfig},
};

mod database;
mod error;
mod representation;
mod routes;

#[rocket::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let content = match fs::read_to_string("./config.toml") {
        Ok(content) => content,
        Err(_) => {
            log::error!("Unable to read config.toml");
            return
        }
    };

    let config: GeneralConfig = toml::from_str(&content).unwrap();

    let _ = rocket::build()
        .manage(DatabaseHolder(Mutex::new(
            Box::new(InMemoryDatabase::new()),
        ))) // Create In-memory database to test the endpoints TODO: Add configurable MongoDB database
        .manage(trustifier_config(&config)) // Trustifier Config
        .manage(config) // Converza config
        .mount("/v1/friends", routes::v1::friends::routes())
        .mount("/v1/auth", routes::v1::auth::routes())
        .mount("/v1/user", routes::v1::user::routes())
        .mount("/", routes![routes::v1::events])
        .mount(
            "/v1/swagger-ui",
            make_swagger_ui(&SwaggerUIConfig {
                urls: vec![
                    UrlObject::new("Authentication API", "../auth/openapi.json"),
                    UrlObject::new("User API", "../user/openapi.json"),
                ],
                ..Default::default()
            }),
        )
        .launch()
        .await
        .unwrap();
}
