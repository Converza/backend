use rocket::{serde::Deserialize, tokio::sync::broadcast::Sender};
use schemars::JsonSchema;

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
    pub event_sender: Option<Sender<Event>>,
    pub friend_requests: Vec<String>,
    pub friends: Vec<String>,
}
