use rocket::{
    response::stream::{Event, EventStream},
    tokio::{
        select,
        sync::broadcast::{channel, error::RecvError},
    },
    Shutdown, State,
};
use trustifier::models::account::Session;

use crate::database::DatabaseHolder;

pub mod auth;
pub mod friends;
pub mod user;

#[get("/events")]
pub async fn events(
    session: Session,
    mut end: Shutdown,
    database: &State<DatabaseHolder>,
) -> EventStream![] {
    let mut database = database.inner().0.lock();
    let user = database.find_account_by_id_mut(&session.user_id).unwrap();
    if user.properties.event_sender.is_none() {
        user.properties.event_sender = Some(channel::<crate::representation::Event>(1024).0);
    }
    let mut rx = user.properties.event_sender.as_ref().unwrap().subscribe();
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
