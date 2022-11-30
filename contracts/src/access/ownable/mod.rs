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

pub use crate::{
    ownable,
    traits::ownable::*,
};
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Storage,
        ZERO_ADDRESS,
    },
};
pub use ownable::Internal as _;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub owner: AccountId,
    pub _reserved: Option<()>,
}

/// Throws if called by any account other than the owner.
#[modifier_definition]
pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<OwnableError>,
{
    if instance.data().owner != T::env().caller() {
        return Err(From::from(OwnableError::CallerIsNotOwner))
    }
    body(instance)
}

impl<T: Storage<Data>> Ownable for T {
    default fn owner(&self) -> AccountId {
        self.data().owner.clone()
    }

    #[modifiers(only_owner)]
    default fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        let old_owner = self.data().owner.clone();
        self.data().owner = ZERO_ADDRESS.into();
        self._emit_ownership_transferred_event(Some(old_owner), None);
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        if new_owner.is_zero() {
            return Err(OwnableError::NewOwnerIsZero)
        }
        let old_owner = self.data().owner.clone();
        self.data().owner = new_owner.clone();
        self._emit_ownership_transferred_event(Some(old_owner), Some(new_owner));
        Ok(())
    }
}

pub trait Internal {
    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>);

    fn _init_with_owner(&mut self, owner: AccountId);
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>) {}

    default fn _init_with_owner(&mut self, owner: AccountId) {
        self.data().owner = owner;
        self._emit_ownership_transferred_event(None, Some(owner));
    }
}
