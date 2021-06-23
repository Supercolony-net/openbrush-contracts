use brush::{
    traits::{InkStorage, AccountId},
};
use crate::traits::OwnableError;

#[brush::internal_trait_definition]
pub trait OwnableStorage: InkStorage {
    fn _owner(&self) -> & AccountId;
    fn _owner_mut(&mut self) -> &mut AccountId;
}

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Ownable: OwnableStorage {
    fn only_owner(&self) {
        assert_eq!(self._owner(), &Self::env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
    }

    /// User must override this method in their contract.
    fn emit_ownership_transferred_event(&self, _previous_owner: Option<AccountId>, _new_owner: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        *self._owner_mut() = owner;
        self.emit_ownership_transferred_event(None, Some(owner));
    }

    fn owner(&self) -> AccountId {
        self._owner().clone()
    }

    fn renounce_ownership(&mut self) {
        self.only_owner();

        let old_owner = self.owner();
        *self._owner_mut() = ZERO_ADDRESS.into();
        self.emit_ownership_transferred_event(Some(old_owner), None);
    }

    fn transfer_ownership(&mut self, new_owner: AccountId) {
        assert_ne!(new_owner, ZERO_ADDRESS.into(), "{}", OwnableError::NewOwnerIsZero.as_ref());
        let old_owner = self.owner();
        *self._owner_mut() = new_owner;
        self.emit_ownership_transferred_event(Some(old_owner), Some(self.owner()));
    }
}