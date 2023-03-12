use rocket::{serde::json::Value, Route, State};
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::_serde_json::json;
use trustifier::models::account::Session;

use crate::{database::DatabaseHolder, error::Error};

/// This API allows the user to change his username to the specified name
#[openapi(tag = "User")]
#[post("/change_username", data = "<username>")]
fn change_username(
    session: Session,
    username: String,
    database: &State<DatabaseHolder>,
) -> Result<Value, Error> {
    let mut database = database.inner().0.lock();
    let user = database.find_account_by_id_mut(&session.user_id)?;
    user.username_history.push(username);
    Ok(json!({
        "code": 200,
        "status": "Username changed",
    }))
}

pub fn routes() -> Vec<Route> {
    openapi_get_routes![change_username]
}
