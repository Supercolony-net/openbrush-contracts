pub use crate::{
    psp721::*,
    traits::psp721::extensions::burnable::*,
};
use brush::traits::AccountId;

impl<T: PSP721Internal> PSP721Burnable for T {
    default fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._burn_from(account, id)
    }
}
