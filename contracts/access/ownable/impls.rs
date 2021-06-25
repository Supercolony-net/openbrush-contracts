use brush::{
    traits::{InkStorage, AccountId},
};
use crate::traits::OwnableError;
pub use ownable_derive::OwnableStorage;

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

    fn _init_with_owner(&mut self, owner: AccountId) {
        *self._owner_mut() = owner;
        // TODO: Emit event
    }

    fn owner(&self) -> AccountId {
        self._owner().clone()
    }

    fn renounce_ownership(&mut self) {
        // TODO: Emit event
        *self._owner_mut() = ZERO_ADDRESS.into();
    }

    fn transfer_ownership(&mut self, new_owner: AccountId) {
        assert_ne!(new_owner, ZERO_ADDRESS.into(), "{}", OwnableError::NewOwnerIsZero.as_ref());
        // TODO: Emit event
        *self._owner_mut() = new_owner;
    }
}