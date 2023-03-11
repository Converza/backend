use parking_lot::Mutex;

use crate::{error::Error, representation::models::User};

pub mod memory;

pub struct DatabaseHolder(pub Mutex<Box<dyn Database>>);

pub trait Database: Send {
    fn register_user(&mut self, user: User) -> Result<(), Error>;

    fn find_user_by_id_mut(&mut self, id: &str) -> Result<&mut User, Error>;

    fn find_user_by_id(&self, id: &str) -> Result<&User, Error>;

    fn find_user_by_email(&self, email: &str) -> Result<&User, Error>;

    fn find_user_by_name(&self, username: &str) -> Result<&User, Error>;
}
