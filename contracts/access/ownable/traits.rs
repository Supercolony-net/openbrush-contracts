use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        InkStorage,
        ZERO_ADDRESS,
    },
};
use ink_storage::traits::SpreadLayout;
pub use ownable_derive::OwnableStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct OwnableData {
    pub owner: AccountId,
}

declare_storage_trait!(OwnableStorage, OwnableData);

/// The Ownable error type. Contract will throw one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum OwnableError {
    CallerIsNotOwner,
    NewOwnerIsZero,
}

/// Throws if called by any account other than the owner.
#[modifier_definition]
pub fn only_owner<T, F, ReturnType>(instance: &mut T, body: F) -> ReturnType
where
    T: OwnableStorage,
    F: FnOnce(&mut T) -> ReturnType,
{
    assert_eq!(
        instance.get().owner,
        T::env().caller(),
        "{}",
        OwnableError::CallerIsNotOwner.as_ref()
    );
    body(instance)
}

/// Contract module which provides a basic access control mechanism, where
/// there is an account (an owner) that can be granted exclusive access to
/// specific functions.
///
/// This module is used through embedding of `OwnableData` and implementation of `Ownable` and
/// `OwnableStorage` traits. It will make available the modifier `only_owner`, which can be applied
/// to your functions to restrict their use to the owner.
#[brush::trait_definition]
pub trait Ownable: OwnableStorage {
    /// Returns the address of the current owner.
    #[ink(message)]
    fn owner(&self) -> AccountId {
        self.get().owner.clone()
    }

    /// Leaves the contract without owner. It will not be possible to call
    /// `onlyOwner` functions anymore. Can only be called by the current owner.
    ///
    /// NOTE: Renouncing ownership will leave the contract without an owner,
    /// thereby removing any functionality that is only available to the owner.
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner
    #[ink(message)]
    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self) {
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = ZERO_ADDRESS.into();
        self._emit_ownership_transferred_event(Some(old_owner), None);
    }

    /// Transfers ownership of the contract to a `new_owner`.
    /// Can only be called by the current owner.
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner.
    ///
    /// Panics with `NewOwnerIsZero` error if new owner's address is zero.
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
