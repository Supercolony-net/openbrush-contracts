pub use brush::modifiers;
use brush::{
    traits::{InkStorage, AccountId},
};
pub use ownable_derive::OwnableStorage;

#[brush::storage_trait]
pub trait OwnableStorage: InkStorage {
    fn _owner(&self) -> & AccountId;
    fn _owner_mut(&mut self) -> &mut AccountId;
}

#[derive(strum_macros::AsRefStr)]
pub enum OwnableError {
    CallerIsNotOwner,
    NewOwnerIsZero,
}

#[brush::trait_definition]
pub trait IOwnable:  {
    #[ink(message)]
    fn owner(&self) -> AccountId {
        self._owner().clone()
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self) {
        // TODO: Emit event
        *self._owner_mut() = [0; 32].into();
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: AccountId) {
        assert_ne!(new_owner, [0; 32].into(), "{}", OwnableError::NewOwnerIsZero.as_ref());
        // TODO: Emit event
        *self._owner_mut() = new_owner;
    }
    
    // Helper functions

    fn only_owner(&self) {
        assert_eq!(self._owner(), &Self::env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
    }

    fn _init_with_owner(&mut self, owner: AccountId) {
        *self._owner_mut() = owner;
        // TODO: Emit event
    }
}
