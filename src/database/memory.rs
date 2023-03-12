use trustifier::models::account::Account;

use crate::{database::Database, error::Error, representation::models::AccountProperties};

pub struct InMemoryDatabase {
    accounts: Vec<Account<AccountProperties>>,
}

impl Database for InMemoryDatabase {
    fn register_account(&mut self, account: Account<AccountProperties>) -> Result<(), Error> {
        self.accounts.push(account);
        Ok(())
    }

    fn find_account_by_id_mut(
        &mut self,
        id: &str,
    ) -> Result<&mut Account<AccountProperties>, Error> {
        self.accounts
            .iter_mut()
            .find(|account| account.id.eq(id))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_account_by_id(&self, id: &str) -> Result<&Account<AccountProperties>, Error> {
        self.accounts
            .iter()
            .find(|account| account.id.eq(id))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_account_by_email(&self, email: &str) -> Result<&Account<AccountProperties>, Error> {
        self.accounts
            .iter()
            .find(|account| account.email.eq(email))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_account_by_email_mut(
        &mut self,
        email: &str,
    ) -> Result<&mut Account<AccountProperties>, Error> {
        self.accounts
            .iter_mut()
            .find(|account| account.email.eq(email))
            .ok_or(Error::NotFound(String::from("User")))
    }

    fn find_account_by_name(&self, username: &str) -> Result<&Account<AccountProperties>, Error> {
        self.accounts
            .iter()
            .find(|account| account.current_username().eq(username))
            .ok_or(Error::NotFound(String::from("User")))
    }
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }
}
