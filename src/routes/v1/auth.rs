use chrono::Utc;
use rocket::{
    serde::json::{serde_json::json, Json, Value},
    Route, State,
};
use rocket_okapi::{openapi, openapi_get_routes};
use uuid::Uuid;

use crate::{
    database::DatabaseHolder,
    error::Error,
    representation::{
        config::GeneralConfig,
        models::{LoginRequest, RegistrationRequest, Session, User},
    },
};

/// This is the endpoint for the user login. If you have created an account in the frontend or
/// over other external APIs, you can login here with your email and password.
#[openapi(tag = "Authentication")]
#[post("/login", data = "<request>")]
async fn login(
    request: Json<LoginRequest>,
    database: &State<DatabaseHolder>,
    config: &State<GeneralConfig>,
) -> Result<Value, Error> {
    let database = database.inner().0.lock();
    let user = database.find_user_by_email(&request.email).map_err(|_| Error::InvalidCredentials)?;
    if !user.is_password_equal(&request.password, &config.hashing)? {
        return Err(Error::InvalidCredentials)
    }

    // TODO: Implement lockout (If you fail too many tries, the login get blocked temporary)
    // TODO: Implement MFA

    let session = Session {
        user_id: user.id.clone(),
        id: Uuid::new_v4().to_string(),
        exp: Utc::now()
            .checked_add_signed(chrono::Duration::days(config.auth.session_lifetime as i64))
            .expect("Invalid timestamp")
            .timestamp(),
    };
    let jwt = session.to_jwt(&config.auth)?;

    Ok(json!({
        "code": 200,
        "status": "Logged in",
        "token": jwt
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
    config: &State<GeneralConfig>,
) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    if database.find_user_by_email(&request.email).is_ok() {
        return Err(Error::AlreadyExisting(String::from("User")))
    }

    if database.find_user_by_name(&request.username).is_ok() {
        return Err(Error::AlreadyExisting(String::from("User")))
    }

    if let Err(error) = config.password.check_password(&request.password) {
        return Err(Error::WeakPassword(error.to_string()))
    }

    // TODO: Invite system

    database
        .register_user(User::new(
            request.email.clone(),
            request.username.clone(),
            request.password.clone(),
            &config.hashing,
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
