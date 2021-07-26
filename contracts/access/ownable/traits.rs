use brush::{modifiers, modifier_definition};
use brush::traits::{AccountIdExt, ZERO_ADDRESS};
use brush::traits::{InkStorage, AccountId};
use brush::declare_storage_trait;
use ink_storage::{
    traits::{PackedLayout, SpreadLayout},
};
pub use ownable_derive::OwnableStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct OwnableData {
    pub owner: AccountId,
}

declare_storage_trait!(OwnableStorage, OwnableData);

#[derive(strum_macros::AsRefStr)]
pub enum OwnableError {
    CallerIsNotOwner,
    NewOwnerIsZero,
}

#[modifier_definition]
pub fn only_owner<T, F, ReturnType>(instance: &mut T, mut body: F) -> ReturnType
    where
        T: OwnableStorage,
        F: FnMut(&mut T) -> ReturnType,
{
    assert_eq!(instance.get().owner, T::env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
    body(instance)
}

#[brush::trait_definition]
pub trait IOwnable: OwnableStorage + Sized {
    #[ink(message)]
    fn owner(&self) -> AccountId {
        self.get().owner.clone()
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self) {
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = ZERO_ADDRESS.into();
        self._emit_ownership_transferred_event(Some(old_owner), None);
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: AccountId) {
        assert!(!new_owner.is_zero(), "{}", OwnableError::NewOwnerIsZero.as_ref());
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = new_owner.clone();
        self._emit_ownership_transferred_event(Some(old_owner), Some(new_owner));
    }
    
    // Helper functions

    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous_owner: Option<AccountId>, _new_owner: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        self.get_mut().owner = owner;
        self._emit_ownership_transferred_event(None, Some(owner));
    }
}
