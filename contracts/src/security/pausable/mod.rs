// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::traits::pausable::*;
pub use derive::PausableStorage;
use openbrush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::AccountId,
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PausableData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PausableData {
    pub paused: bool,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PausableStorage);

/// Modifier to make a function callable only when the contract is paused.
#[modifier_definition]
pub fn when_paused<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: PausableStorage<Data = PausableData>,
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
    T: PausableStorage<Data = PausableData>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<PausableError>,
{
    if instance.get().paused {
        return Err(From::from(PausableError::Paused))
    }
    body(instance)
}

impl<T: PausableStorage<Data = PausableData>> Pausable for T {
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

impl<T: PausableStorage<Data = PausableData>> PausableInternal for T {
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
