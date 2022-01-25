pub use crate::{
    psp721::*,
    traits::psp721::extensions::mintable::*,
};
use brush::traits::AccountId;

impl<T: PSP721Internal> PSP721Mintable for T {
    default fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._mint_to(account, id)
    }
}
