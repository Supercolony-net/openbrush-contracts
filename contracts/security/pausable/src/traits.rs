use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        InkStorage,
    },
};
pub use common::errors::PausableError;
use ink_storage::traits::SpreadLayout;
pub use pausable_derive::PausableStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PausableData {
    pub paused: bool,
}

declare_storage_trait!(PausableStorage, PausableData);

/// Modifier to make a function callable only when the contract is paused.
#[modifier_definition]
pub fn when_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: PausableStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if !instance.get().paused {
        return Err(From::from(PausableError::NotPaused))
    }
    body(instance)
}

/// Modifier to make a function callable only when the contract is not paused.
#[modifier_definition]
pub fn when_not_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: PausableStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if instance.get().paused {
        return Err(From::from(PausableError::Paused))
    }
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
    fn _pause<E: From<PausableError>>(&mut self) -> Result<(), E> {
        self.get_mut().paused = true;
        self._emit_paused_event(Self::env().caller());
        Ok(())
    }

    /// Returns to normal state.
    ///
    /// On success a `Unpaused` event is emitted.
    #[modifiers(when_paused)]
    fn _unpause<E: From<PausableError>>(&mut self) -> Result<(), E> {
        self.get_mut().paused = false;
        self._emit_unpaused_event(Self::env().caller());
        Ok(())
    }
}
