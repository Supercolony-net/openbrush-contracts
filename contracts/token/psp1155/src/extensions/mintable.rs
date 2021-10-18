/// Extension of [`PSP1155`] that allows minting of new tokens
use crate::traits::*;

use brush::traits::{
    AccountId,
    Balance,
};

#[brush::trait_definition]
pub trait PSP1155Mintable: PSP1155 {
    /// Mints `amount` tokens of token type `id` to the user
    ///
    /// See [`PSP1155::_mint`].
    #[ink(message)]
    fn mint(&mut self, id: Id, amount: Balance) {
        self._mint(Self::env().caller(), id, amount);
    }

    /// Mints `amount` tokens of token type `id` to `to`
    ///
    /// See [`PSP1155::_mint`].
    #[ink(message)]
    fn mint_to(&mut self, to: AccountId, id: Id, amount: Balance) {
        self._mint(to, id, amount);
    }
}
