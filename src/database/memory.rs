use crate::{database::Database, error::Error, representation::models::User};

pub struct InMemoryDatabase {
    users: Vec<User>,
}

impl Database for InMemoryDatabase {
    fn register_user(&mut self, user: User) -> Result<(), Error> {
        self.users.push(user);
        Ok(())
    }

    fn find_user_by_id_mut(&mut self, id: &str) -> Result<&mut User, Error> {
        self.users
            .iter_mut()
            .find(|user| user.id.eq(id))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_user_by_id(&self, id: &str) -> Result<&User, Error> {
        self.users
            .iter()
            .find(|user| user.id.eq(id))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_user_by_email(&self, email: &str) -> Result<&User, Error> {
        self.users
            .iter()
            .find(|user| user.email.eq(email))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_user_by_name(&self, username: &str) -> Result<&User, Error> {
        self.users
            .iter()
            .find(|user| user.username.eq(username))
            .ok_or(Error::NotFound(String::from("User")))
    }
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}
