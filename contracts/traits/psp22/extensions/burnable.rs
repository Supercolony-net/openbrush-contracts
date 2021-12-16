/// Extension of [`PSP22`] that allows token holders to destroy both their own
/// tokens and those that they have an allowance for.
pub use crate::traits::errors::PSP22Error;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::wrapper]
pub type PSP22BurnableRef = dyn PSP22Burnable;

#[brush::trait_definition]
pub trait PSP22Burnable {
    /// Destroys `amount` tokens from the caller.
    ///
    /// See [`PSP22::_burn`].
    #[ink(message)]
    fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error>;

    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22::_burn_from`].
    #[ink(message)]
    fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
