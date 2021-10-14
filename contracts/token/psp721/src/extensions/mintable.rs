/// Extension of [`PSP721`] that exposes the mint function
use crate::traits::*;

use brush::traits::AccountId;

#[brush::trait_definition]
pub trait PSP721Mintable: PSP721 {
    /// Mints a new token with 'id'.
    ///
    /// See [`PSP721::_mint`].
    #[ink(message)]
    fn mint(&mut self, id: Id) {
        self._mint(id);
    }

    /// Mints a new token with 'id' to 'account'
    ///
    /// or to transfer token with 'id'
    /// See [`PSP721::_mint_to`].
    #[ink(message)]
    fn mint_to(&mut self, account: AccountId, id: Id) {
        self._mint_to(account, id);
    }
}
