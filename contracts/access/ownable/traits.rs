pub use brush::modifiers;
pub use ink_lang::{Env, StaticEnv};
pub use brush::traits::{AccountIdExt, ZERO_ADDRESS};
pub use ownable_derive::OwnableStorage;

// We don't need to expose it, because ink! will define AccountId and StaticEnv itself.
use brush::traits::{InkStorage, AccountId};

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
pub trait IOwnable: OwnableStorage {
    #[ink(message)]
    fn owner(&self) -> AccountId {
        self._owner().clone()
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self) {
        let old_owner = self._owner().clone();
        *self._owner_mut() = ZERO_ADDRESS.into();
        self._emit_ownership_transferred_event(Some(old_owner), None);
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: AccountId) {
        assert!(!new_owner.is_zero(), "{}", OwnableError::NewOwnerIsZero.as_ref());
        let old_owner = self._owner().clone();
        *self._owner_mut() = new_owner.clone();
        self._emit_ownership_transferred_event(Some(old_owner), Some(new_owner));
    }
    
    // Helper functions

    fn only_owner(&self) {
        assert_eq!(self._owner(), &Self::env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
    }

    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous_owner: Option<AccountId>, _new_owner: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        *self._owner_mut() = owner;
        self._emit_ownership_transferred_event(None, Some(owner));
    }
}
