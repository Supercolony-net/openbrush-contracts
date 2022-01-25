/// Extension of [`PSP1155`] that allows minting of new tokens
use crate::traits::psp1155::Id;
use crate::traits::psp1155::PSP1155Error;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type PSP1155MintableRef = dyn PSP1155Mintable;

#[brush::trait_definition]
pub trait PSP1155Mintable {
    /// Mints `amount` tokens of token type `id` to `to`
    ///
    /// See [`PSP1155::_mint_to`].
    #[ink(message)]
    fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error>;
}