pub use crate::{
    psp34::*,
    traits::psp34::extensions::mintable::*,
};
use brush::traits::AccountId;

impl<T: PSP34Internal> PSP34Mintable for T {
    default fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._mint_to(account, id)
    }
}
