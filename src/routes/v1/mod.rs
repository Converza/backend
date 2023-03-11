use rocket::{response::stream::{Event, EventStream}, Shutdown, State};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::error::RecvError;
use crate::database::DatabaseHolder;
use crate::representation::models::Session;

pub mod friends;
pub mod auth;
pub mod user;

#[get("/events")]
pub async fn events(session: Session, mut end: Shutdown, database: &State<DatabaseHolder>) -> EventStream![] {
    let database = database.inner().0.lock();
    let user = database.find_user_by_id(&session.user_id).unwrap();
    let mut rx = user.sender.subscribe();
    EventStream! {
        loop {
            let event = select! {
                event = rx.recv() => match event {
                    Ok(event) => event,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue
                },
                _ = &mut end => break
            };

            yield Event::json(&event);
        }
    }
}
