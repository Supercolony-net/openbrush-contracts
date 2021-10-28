use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        InkStorage,
    },
};
use ink_storage::traits::SpreadLayout;
pub use pausable_derive::PausableStorage;
use scale::{
    Decode,
    Encode,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PausableData {
    pub paused: bool,
}

declare_storage_trait!(PausableStorage, PausableData);

/// The Pausable error type. Contract will throw one of this errors.
#[derive(Debug, strum_macros::AsRefStr, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PausableError {
    Paused,
    NoPaused,
}

/// Modifier to make a function callable only when the contract is paused.
#[modifier_definition]
pub fn when_paused<T, F, ReturnType>(instance: &mut T, body: F) -> ReturnType
where
    T: PausableStorage,
    F: FnOnce(&mut T) -> ReturnType,
{
    assert!(instance.get().paused, "{}", PausableError::NoPaused.as_ref());
    body(instance)
}

/// Modifier to make a function callable only when the contract is not paused.
#[modifier_definition]
pub fn when_not_paused<T, F, ReturnType>(instance: &mut T, body: F) -> ReturnType
where
    T: PausableStorage,
    F: FnOnce(&mut T) -> ReturnType,
{
    assert!(!instance.get().paused, "{}", PausableError::Paused.as_ref());
    body(instance)
}

/// Contract module, which allows children to implement an emergency stop
/// mechanism that an authorized account can trigger.
///
/// This module is used through the embedding of `PausableData` and implementation of `Pausable` and
/// `PausableStorage` traits. It will make available the modifier `when_not_paused` and `when_paused`,
/// which can be applied to your functions to restrict their usage.
#[brush::trait_definition]
pub trait Pausable: PausableStorage {
    /// Returns true if the contract is paused, and false otherwise.
    #[ink(message)]
    fn paused(&self) -> bool {
        self.get().paused
    }

    // Helper functions

    /// User must override this method in their contract.
    fn _emit_paused_event(&self, _account: AccountId) {}

    /// User must override this method in their contract.
    fn _emit_unpaused_event(&self, _account: AccountId) {}

    /// Triggers stopped state.
    ///
    /// On success a `Paused` event is emitted.
    #[modifiers(when_not_paused)]
    fn _pause(&mut self) {
        self.get_mut().paused = true;
        self._emit_paused_event(Self::env().caller())
    }

    /// Returns to normal state.
    ///
    /// On success a `Unpaused` event is emitted.
    #[modifiers(when_paused)]
    fn _unpause(&mut self) {
        self.get_mut().paused = false;
        self._emit_unpaused_event(Self::env().caller())
    }
}
