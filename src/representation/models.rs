use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use okapi::{
    openapi3::{Parameter, ParameterValue},
    Map,
};
use rand::{distributions::Alphanumeric, Rng};
use rocket::{
    http::Status,
    outcome::Outcome::Success,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};
use rocket::tokio::sync::broadcast::{channel, Sender};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use schemars::JsonSchema;
use uuid::Uuid;

use crate::{
    error::Error,
    representation::config::{AuthConfig, GeneralConfig, HashConfig},
};
use crate::representation::Event;

#[derive(Clone, JsonSchema, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, JsonSchema, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Default)]
pub struct AccountProperties {

}