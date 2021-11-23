/// Extension of [`PSP22`] that allows create `amount` tokens
/// and assigns them to `account`, increasing the total supply
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::wrapper]
pub type PSP22MintableCaller = dyn PSP22Mintable + PSP22;

#[brush::trait_definition]
pub trait PSP22Mintable: PSP22 {
    /// Minting `amount` tokens to the account.
    ///
    /// See [`PSP22::_mint`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint(account, amount)
    }
}
