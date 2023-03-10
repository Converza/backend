use rocket::{
    serde::json::{
        serde_json::{json, Map, Number},
        Value,
    },
    Route, State,
};
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::_serde_json::to_value;
use trustifier::models::account::Session;

use crate::{database::DatabaseHolder, error::Error, representation::Event};

/// This endpoints request a friendship with the specified user. This is an authenticated endpoint.
#[openapi]
#[post("/request", data = "<id>")]
fn request(session: Session, id: String, database: &State<DatabaseHolder>) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    let user = database.find_account_by_id(&session.user_id)?;
    let other_user = database.find_account_by_id(&id)?;

    if other_user.properties.friend_requests.contains(&user.id) {
        return Err(Error::AlreadyExisting(String::from("Friend Request")))
    }

    if user.properties.friends.contains(&other_user.id) {
        return Err(Error::BadRequest(String::from(
            "That guy is already your friend!",
        )))
    }
    let user_id = user.id.clone();
    let other_user = database.find_account_by_id_mut(&id)?;

    // Add friend and send to the other user the information
    other_user.properties.friend_requests.push(user_id.clone());

    if let Some(sender) = other_user.properties.event_sender.as_ref() {
        sender.send(Event::FriendRequest(user_id)).map_err(|_| {
            Error::Server(String::from("Unable to push friend request event to user!"))
        })?;
    }
    Ok(json!({
        "status": "Friend Request sent",
        "code": 200
    }))
}

/// This endpoint accepts
#[openapi]
#[post("/accept_request", data = "<id>")]
fn accept_request(
    session: Session,
    id: String,
    database: &State<DatabaseHolder>,
) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    let other_user = database.find_account_by_id(&id)?.id.clone();
    let user = database.find_account_by_id_mut(&session.user_id)?;

    if !user.properties.friend_requests.contains(&other_user) {
        return Err(Error::NotFound(String::from("Friend Request")))
    }

    user.properties.friend_requests.remove(
        user.properties
            .friend_requests
            .iter()
            .position(|r| r == &other_user)
            .unwrap(),
    );

    // Add to friends on acceptor
    user.properties.friends.push(other_user);
    if let Some(sender) = user.properties.event_sender.as_ref() {
        sender
            .send(Event::FriendRequestAccepted(user.id.clone()))
            .map_err(|_| {
                Error::Server(String::from(
                    "Unable to push friend request accepted event to user!",
                ))
            })?;
    }

    let user_id = user.id.clone();
    let other_user = database.find_account_by_id_mut(&id)?;

    other_user.properties.friends.push(user_id.clone());
    if let Some(sender) = other_user.properties.event_sender.as_ref() {
        sender
            .send(Event::FriendRequestAccepted(user_id))
            .map_err(|_| {
                Error::Server(String::from(
                    "Unable to push friend request accepted event to user!",
                ))
            })?;
    }

    Ok(json!({
        "status": "Friend Request accepted",
        "code": 200
    }))
}

/// This endpoint returns an list of all friends. This is an authenticated endpoint.
#[openapi]
#[get("/list")]
fn list(session: Session, database: &State<DatabaseHolder>) -> Result<Value, Error> {
    let database = database.inner().0.lock();
    let user = database.find_account_by_id(&session.user_id)?;

    let mut map = Map::new();
    map.insert(String::from("code"), Value::Number(Number::from(200)));
    map.insert(
        String::from("status"),
        Value::String(String::from("Successfully")),
    );
    map.insert(
        String::from("friends"),
        to_value(user.properties.friends.clone()).unwrap(),
    );

    Ok(Value::Object(map))
}

pub fn routes() -> Vec<Route> {
    openapi_get_routes![list, accept_request, request]
}
