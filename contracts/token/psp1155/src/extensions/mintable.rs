/// Extension of [`PSP1155`] that allows minting of new tokens
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::trait_definition]
pub trait PSP1155Mintable: PSP1155 {
    /// Mints `amount` tokens of token type `id` to the caller
    ///
    /// See [`PSP1155::_mint_to`].
    #[ink(message)]
    fn mint(&mut self, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._mint_to(Self::env().caller(), ids_amounts)
    }

    /// Mints `amount` tokens of token type `id` to `to`
    ///
    /// See [`PSP1155::_mint_to`].
    #[ink(message)]
    fn mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._mint_to(to, ids_amounts)
    }
}
