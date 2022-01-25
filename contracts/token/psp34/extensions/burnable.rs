pub use crate::{
    psp34::*,
    traits::psp34::extensions::burnable::*,
};
use brush::traits::AccountId;

impl<T: PSP34Internal> PSP34Burnable for T {
    default fn burn(&mut self, id: Id) -> Result<(), PSP34Error> {
        self._burn(id)
    }

    default fn burn_from(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._burn_from(account, id)
    }
}
