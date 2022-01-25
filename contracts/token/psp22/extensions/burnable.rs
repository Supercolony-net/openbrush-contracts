pub use crate::{
    psp22::*,
    traits::psp22::extensions::burnable::*,
};
use brush::traits::{
    AccountId,
    Balance,
    InkStorage,
};

impl<T: PSP22Internal + InkStorage> PSP22Burnable for T {
    default fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(account, amount)
    }
}
