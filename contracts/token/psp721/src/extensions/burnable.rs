/// Extension of [`PSP721`] that allows token holders to destroy their tokens
use crate::traits::*;

use brush::traits::{
    AccountId,
    ZERO_ADDRESS,
};

#[brush::trait_definition]
pub trait PSP721Burnable: IPSP721 {
    /// Destroys token with id equal to 'id'.
    ///
    /// See [`PSP721::_burn`].
    #[ink(message)]
    fn burn(&mut self, id: Id) {
        self._burn(id);
    }

    /// Destroys token with id equal to 'id' from 'account'
    ///
    /// caller must be approved to transfer tokens from 'account'
    /// or to transfer token with 'id'
    #[ink(message)]
    fn burn_from(&mut self, account: AccountId, id: Id) {
        let caller = Self::env().caller();

        let is_approved = self.get_approved(id).unwrap_or(ZERO_ADDRESS.into()) == caller;
        let is_approved_for_all = self.is_approved_for_all(account, caller);

        assert_eq!(
            is_approved || is_approved_for_all,
            true,
            "{}",
            PSP721Error::NotApproved.as_ref()
        );

        self._remove_from(account, id.clone());
        self._emit_transfer_event(account, ZERO_ADDRESS.into(), id);
    }
}
