pub use crate::{
    psp22::*,
    traits::psp22::extensions::mintable::*,
};
use brush::traits::{
    AccountId,
    Balance,
};

impl<T: PSP22Internal> PSP22Mintable for T {
    default fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint(account, amount)
    }
}
