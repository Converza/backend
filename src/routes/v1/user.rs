use rocket::{Route, State};
use rocket::serde::json::Value;
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::_serde_json::json;
use crate::database::DatabaseHolder;
use crate::error::Error;
use crate::representation::models::Session;

/// This API allows the user to change his username to the specified name
#[openapi(tag = "User")]
#[post("/change_username", data = "<username>")]
pub fn change_username(session: Session, username: String, database: &State<DatabaseHolder>) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    let user = database.find_user_by_id_mut(&session.user_id)?;
    user.username = username;
    Ok(json!({
        "code": 200,
        "status": "Username changed",
    }))
}

pub fn routes() -> Vec<Route> {
    openapi_get_routes![change_username]
}
