use rocket::serde::Serialize;

pub mod config;
pub mod models;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Event {
    MessageReceived(String),
    FriendRequest(String),
    FriendRequestAccepted(String),
}
