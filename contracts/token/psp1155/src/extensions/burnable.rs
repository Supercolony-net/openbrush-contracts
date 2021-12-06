/// Extension of [`PSP1155`] that allows token holders to destroy their tokens
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type PSP1155BurnableRef = dyn PSP1155Burnable + PSP1155;

#[brush::trait_definition]
pub trait PSP1155Burnable: PSP1155 {
    /// Destroys `amount` tokens of token type `id` from the caller
    ///
    /// See [`PSP1155::_burn_from`].
    #[ink(message)]
    fn burn(&mut self, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._burn_from(Self::env().caller(), ids_amounts)
    }

    /// Destroys `amount` tokens of token type `id` from `from`
    ///
    /// See [`PSP1155::_burn_from`].
    #[ink(message)]
    fn burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._burn_from(from, ids_amounts)
    }
}
