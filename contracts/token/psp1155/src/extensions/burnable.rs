/// Extension of [`PSP1155`] that allows token holders to destroy their tokens
use crate::traits::*;

use brush::traits::{
    AccountId,
    Balance,
};

#[brush::trait_definition]
pub trait PSP1155Burnable: PSP1155 {
    /// Destroys `amount` tokens of token type `id` from the user
    ///
    /// See [`PSP1155::_burn`].
    #[ink(message)]
    fn burn(&mut self, id: Id, amount: Balance) {
        let caller = Self::env().caller();
        self._burn(caller, id, amount);
    }

    /// Destroys `amount` tokens of token type `id` from 'from'
    ///
    /// See [`PSP1155::_burn`].
    #[ink(message)]
    fn burn_from(&mut self, from: AccountId, id: Id, amount: Balance) {
        let caller = Self::env().caller();

        assert!(
            self.is_approved_for_all(from, caller),
            "{}",
            PSP1155Error::ApproveRequired.as_ref()
        );

        self._burn(from, id, amount);
    }

    #[ink(message)]
    fn burn_batch(&mut self, ids: Vec<Id>, amounts: Vec<Balance>) {
        let caller = Self::env().caller();
        self._burn_batch(caller, ids, amounts);
    }

    #[ink(message)]
    fn burn_batch_from(&mut self, from: AccountId, ids: Vec<Id>, amounts: Vec<Balance>) {
        let caller = Self::env().caller();

        assert!(
            self.is_approved_for_all(from, caller),
            "{}",
            PSP1155Error::ApproveRequired.as_ref()
        );

        self._burn_batch(from, ids, amounts);
    }
}
