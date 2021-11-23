/// Extension of [`PSP22`] that allows token holders to destroy both their own
/// tokens and those that they have an allowance for.
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::wrapper]
pub type PSP22BurnableCaller = dyn PSP22Burnable + PSP22;

#[brush::trait_definition]
pub trait PSP22Burnable: PSP22 {
    /// Destroys `amount` tokens from the caller.
    ///
    /// See [`PSP22::_burn`].
    #[ink(message)]
    fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self._burn(Self::env().caller(), amount)
    }

    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22::_burn_from`].
    #[ink(message)]
    fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(account, amount)
    }
}
