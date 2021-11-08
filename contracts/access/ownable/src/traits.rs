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
pub use common::errors::OwnableError;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct OwnableData {
    pub owner: AccountId,
}

declare_storage_trait!(OwnableStorage, OwnableData);

/// Throws if called by any account other than the owner.
#[modifier_definition]
pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: OwnableStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<OwnableError>,
{
    if instance.get().owner != T::env().caller() {
        return Err(From::from(OwnableError::CallerIsNotOwner))
    }
    body(instance)
}

/// Contract module which provides a basic access control mechanism, where
/// there is an account (an owner) that can be granted exclusive access to
/// specific functions.
///
/// This module is used through the embedding of `OwnableData` and implementation of `Ownable` and
/// `OwnableStorage` traits. It will make the modifier `only_owner` available, which can be applied
/// to your functions to restrict their use to the owner.
#[brush::trait_definition]
pub trait Ownable: OwnableStorage {
    /// Returns the address of the current owner.
    #[ink(message)]
    fn owner(&self) -> AccountId {
        self.get().owner.clone()
    }

    /// Leaves the contract without owner. It will not be possible to call
    /// `only_owner` functions anymore. Can only be called by the current owner.
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
    fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = ZERO_ADDRESS.into();
        self._emit_ownership_transferred_event(Some(old_owner), None);
        Ok(())
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
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        if new_owner.is_zero() {
            return Err(OwnableError::NewOwnerIsZero);
        }
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = new_owner.clone();
        self._emit_ownership_transferred_event(Some(old_owner), Some(new_owner));
        Ok(())
    }

    // Helper functions

    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous_owner: Option<AccountId>, _new_owner: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        self.get_mut().owner = owner;
        self._emit_ownership_transferred_event(None, Some(owner));
    }
}
