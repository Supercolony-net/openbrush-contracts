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
    psp34::*,
    traits::psp34::extensions::enumerable::*,
};
pub use derive::PSP34EnumerableStorage;
use openbrush::{
    storage::MultipleValueMapping,
    traits::{
        AccountId,
        Flush,
    },
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP34EnumerableData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP34EnumerableData {
    pub enumerable: MultipleValueMapping<Option<AccountId>, Id>,
    pub _reserved: Option<()>,
}

pub trait PSP34EnumerableStorage: PSP34Storage + ::openbrush::traits::InkStorage {
    fn get(&self) -> &PSP34EnumerableData;
    fn get_mut(&mut self) -> &mut PSP34EnumerableData;
}

impl<T: PSP34EnumerableStorage + Flush> PSP34Transfer for T {
    default fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error> {
        self._track_id_transfer(from, to, id)
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error> {
        Ok(())
    }
}

pub trait PSP34EnumerableInternal {
    /// Help function that can be called in `_before_token_transfer`. The function tracks moving of
    /// the token between account to update enumerable data.
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both `None`, ``from``'s `id` will be
    /// transferred to `to`.
    /// - When `from` is `None`, `id` will be minted for `to`.
    /// - When `to` is `None`, ``from``'s `id` will be burned.
    fn _track_id_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error>;

    fn _enumerable(&self) -> &PSP34EnumerableData;

    fn _enumerable_mut(&mut self) -> &mut PSP34EnumerableData;
}

impl<T: PSP34EnumerableStorage + Flush> PSP34EnumerableInternal for T {
    default fn _track_id_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error> {
        if from == to {
            return Ok(())
        }

        if from.is_none() {
            self._enumerable_mut()
                .enumerable
                .insert::<Option<&AccountId>, Id>(&None, id);
        } else {
            self._enumerable_mut()
                .enumerable
                .remove_value::<Option<&AccountId>, Id>(&from, id);
        }

        if to.is_none() {
            self._enumerable_mut()
                .enumerable
                .remove_value::<Option<&AccountId>, Id>(&None, id);
        } else {
            self._enumerable_mut()
                .enumerable
                .insert::<Option<&AccountId>, Id>(&to, id);
        }

        Ok(())
    }

    #[inline(always)]
    fn _enumerable(&self) -> &PSP34EnumerableData {
        PSP34EnumerableStorage::get(self)
    }

    #[inline(always)]
    fn _enumerable_mut(&mut self) -> &mut PSP34EnumerableData {
        PSP34EnumerableStorage::get_mut(self)
    }
}

impl<T: PSP34EnumerableStorage + Flush> PSP34Enumerable for T {
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        self._enumerable()
            .enumerable
            .get_value::<Option<&AccountId>>(&Some(&owner), &index)
            .ok_or(PSP34Error::TokenNotExists)
    }

    default fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        self._enumerable()
            .enumerable
            .get_value::<Option<&AccountId>>(&None, &index)
            .ok_or(PSP34Error::TokenNotExists)
    }
}
