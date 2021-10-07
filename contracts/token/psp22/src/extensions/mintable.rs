/// Extension of [`PSP22`] that allows create `amount` tokens
/// and assigns them to `account`, increasing the total supply
/// tokens and those that they have an allowance for, in a way that can be
/// recognized off-chain (via event analysis).
use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::trait_definition]
pub trait PSP22Mintable: PSP22 {
    /// Minting `amount` tokens to the account.
    ///
    /// See [`PSP22::_mint`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) {
        self._mint(account, amount);
    }
}
