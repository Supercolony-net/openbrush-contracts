/// Extension of [`PSP721`] that allows token holders to destroy their tokens
use crate::traits::*;

use brush::traits::AccountId;

#[brush::wrapper]
pub type PSP721BurnableWrapper = dyn PSP721Burnable + PSP721;

#[brush::trait_definition]
pub trait PSP721Burnable: PSP721 {
    /// Destroys token of type `id`.
    ///
    /// See [`PSP721::_burn`].
    #[ink(message)]
    fn burn(&mut self, id: Id) -> Result<(), PSP721Error> {
        self._burn(id)
    }

    /// Destroys token with id equal to `id` from `account`
    ///
    /// Caller must be approved to transfer tokens from `account`
    /// or to transfer token with `id`
    #[ink(message)]
    fn burn_from(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._burn_from(account, id)
    }
}
