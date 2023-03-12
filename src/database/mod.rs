use parking_lot::Mutex;
use trustifier::models::account::Account;

use crate::{error::Error, representation::models::AccountProperties};

pub mod memory;

pub struct DatabaseHolder(pub Mutex<Box<dyn Database>>);

pub trait Database: Send {
    fn register_account(&mut self, account: Account<AccountProperties>) -> Result<(), Error>;

    fn find_account_by_id_mut(
        &mut self,
        id: &str,
    ) -> Result<&mut Account<AccountProperties>, Error>;

    fn find_account_by_id(&self, id: &str) -> Result<&Account<AccountProperties>, Error>;

    fn find_account_by_email(&self, email: &str) -> Result<&Account<AccountProperties>, Error>;

    fn find_account_by_email_mut(
        &mut self,
        email: &str,
    ) -> Result<&mut Account<AccountProperties>, Error>;

    fn find_account_by_name(&self, username: &str) -> Result<&Account<AccountProperties>, Error>;
}
