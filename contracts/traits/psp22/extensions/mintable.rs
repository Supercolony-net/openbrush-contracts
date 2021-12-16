/// Extension of [`PSP22`] that allows create `amount` tokens
/// and assigns them to `account`, increasing the total supply
pub use crate::traits::errors::PSP22Error;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::wrapper]
pub type PSP22MintableRef = dyn PSP22Mintable;

#[brush::trait_definition]
pub trait PSP22Mintable {
    /// Minting `amount` tokens to the account.
    ///
    /// See [`PSP22::_mint`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
