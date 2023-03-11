#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;

use parking_lot::Mutex;
use rocket::fairing::AdHoc;
use rocket_okapi::{
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use simple_logger::SimpleLogger;

use crate::{
    database::{memory::InMemoryDatabase, DatabaseHolder},
    representation::config::GeneralConfig,
};

mod database;
mod error;
mod representation;
mod routes;

#[rocket::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let _ = rocket::build()
        .manage(DatabaseHolder(Mutex::new(
            Box::new(InMemoryDatabase::new()),
        ))) // Create In-memory database to test the endpoints TODO: Add configurable MongoDB database
        .attach(AdHoc::config::<GeneralConfig>()) // Get configuration from Rocket.toml
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
