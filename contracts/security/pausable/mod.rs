pub use crate::traits::pausable::*;
use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::AccountId,
};
pub use derive::PausableStorage;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PausableData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
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

impl<T: PausableStorage> Pausable for T {
    default fn paused(&self) -> bool {
        self.get().paused
    }
}

pub trait PausableInternal {
    /// User must override this method in their contract.
    fn _emit_paused_event(&self, _account: AccountId);

    /// User must override this method in their contract.
    fn _emit_unpaused_event(&self, _account: AccountId);

    /// Triggers stopped state.
    ///
    /// On success a `Paused` event is emitted.
    #[modifiers(when_not_paused)]
    fn _pause<E: From<PausableError>>(&mut self) -> Result<(), E>;

    /// Returns to normal state.
    ///
    /// On success a `Unpaused` event is emitted.
    #[modifiers(when_paused)]
    fn _unpause<E: From<PausableError>>(&mut self) -> Result<(), E>;
}

impl<T: PausableStorage> PausableInternal for T {
    default fn _emit_paused_event(&self, _account: AccountId) {}

    default fn _emit_unpaused_event(&self, _account: AccountId) {}

    #[modifiers(when_not_paused)]
    default fn _pause<E: From<PausableError>>(&mut self) -> Result<(), E> {
        self.get_mut().paused = true;
        self._emit_paused_event(Self::env().caller());
        Ok(())
    }

    #[modifiers(when_paused)]
    default fn _unpause<E: From<PausableError>>(&mut self) -> Result<(), E> {
        self.get_mut().paused = false;
        self._emit_unpaused_event(Self::env().caller());
        Ok(())
    }
}
