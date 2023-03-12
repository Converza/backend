use rocket::{
    serde::json::{serde_json::json, Json, Value},
    Route, State,
};
use rocket_okapi::{openapi, openapi_get_routes};
use trustifier::{config::TrustifierConfig, models::account::Account};

use crate::{
    database::DatabaseHolder,
    error::Error,
    representation::models::{LoginRequest, RegistrationRequest},
};

/// This is the endpoint for the user login. If you have created an account in the frontend or
/// over other external APIs, you can login here with your email and password.
#[openapi(tag = "Authentication")]
#[post("/login", data = "<request>")]
async fn login(
    request: Json<LoginRequest>,
    database: &State<DatabaseHolder>,
    trustifier_config: &State<TrustifierConfig>,
) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    let user = database
        .find_account_by_email_mut(&request.email)
        .map_err(|_| Error::InvalidCredentials)?;
    let session = user.login(trustifier_config, request.password.clone())?;

    Ok(json!({
        "code": 200,
        "status": "Logged in",
        "token": session.to_jwt(&trustifier_config.password_config)?
    }))
}

/// This is the endpoint for the user registration. If you create an account in the frontend or
/// send over an external API a request to this application, a few checks will run and if they
/// all are successfully, your account get registered.
///
/// You only need to send your username, E-mail and password and if it's needed an invite.
#[openapi(tag = "Authentication")]
#[post("/register", data = "<request>")]
async fn register(
    request: Json<RegistrationRequest>,
    database: &State<DatabaseHolder>,
    config: &State<TrustifierConfig>,
) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    if database.find_account_by_email(&request.email).is_ok() {
        return Err(Error::AlreadyExisting(String::from("User")))
    }

    if database.find_account_by_name(&request.username).is_ok() {
        return Err(Error::AlreadyExisting(String::from("User")))
    }

    // TODO: Invite system

    database
        .register_account(Account::new(
            &config.password_config,
            request.email.clone(),
            request.username.clone(),
            request.password.clone(),
        )?)
        .unwrap();
    Ok(json!({
        "code": 200,
        "status": "Registered"
    }))
}

pub fn routes() -> Vec<Route> {
    openapi_get_routes![register, login]
}
