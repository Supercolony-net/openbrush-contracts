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
    psp35::*,
    traits::psp35::extensions::enumerable::*,
};
pub use derive::PSP35EnumerableStorage;
use ink_prelude::vec::Vec;
use openbrush::{
    storage::{
        Mapping,
        MultiMapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Flush,
    },
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP35EnumerableData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP35EnumerableData {
    pub enumerable: MultiMapping<Option<AccountId>, Id, EnumerableKey>,
    pub total_balance: Mapping<Id, Balance>,
    pub _reserved: Option<()>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub trait PSP35EnumerableStorage: PSP35Storage + ::openbrush::traits::InkStorage {
    fn get(&self) -> &PSP35EnumerableData;
    fn get_mut(&mut self) -> &mut PSP35EnumerableData;
}

impl<T: PSP35EnumerableStorage + Flush> PSP35Transfer for T {
    default fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP35Error> {
        for (id, amount) in ids {
            self._track_id_transfer(from, to, id, amount)?;
        }
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP35Error> {
        Ok(())
    }
}

pub trait PSP35EnumerableInternal {
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
        amount: &Balance,
    ) -> Result<(), PSP35Error>;

    fn _enumerable(&self) -> &PSP35EnumerableData;

    fn _enumerable_mut(&mut self) -> &mut PSP35EnumerableData;

    fn _total_balance_of_id(&self, id: &Id) -> Balance;
}

impl<T: PSP35EnumerableStorage + PSP35Storage + Flush> PSP35EnumerableInternal for T {
    default fn _track_id_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
        amount: &Balance,
    ) -> Result<(), PSP35Error> {
        if from == to {
            return Ok(())
        }

        if from.is_none() {
            if !self._enumerable().enumerable.contains_value(&None, id) {
                self._enumerable_mut().enumerable.insert(&None, id);
            }

            let initial_balance = self._total_balance_of_id(id);

            self._enumerable_mut()
                .total_balance
                .insert(id, &(initial_balance + amount));
        } else {
            let from = from.unwrap();
            let initial_balance = self._balance_of_or_zero(from, id);

            if initial_balance < *amount {
                return Err(PSP35Error::InsufficientBalance)
            }

            if self._balance_of_or_zero(from, id) == *amount {
                self._enumerable_mut().enumerable.remove_value(&Some(from), id);
            }
        }

        if to.is_none() {
            if self._total_balance_of_id(id) < *amount {
                return Err(PSP35Error::InsufficientBalance)
            }

            if self._total_balance_of_id(id) == *amount {
                self._enumerable_mut().enumerable.remove_value(&None, id);
                self._enumerable_mut().total_balance.remove(id);
            } else {
                let initial_balance = self._total_balance_of_id(id);
                self._enumerable_mut()
                    .total_balance
                    .insert(id, &(initial_balance - amount));
            }
        } else {
            let to = to.unwrap();
            if self._balance_of_or_zero(to, id) == 0 {
                self._enumerable_mut().enumerable.insert(&Some(to), id);
            }
        }

        Ok(())
    }

    default fn _enumerable(&self) -> &PSP35EnumerableData {
        PSP35EnumerableStorage::get(self)
    }

    default fn _enumerable_mut(&mut self) -> &mut PSP35EnumerableData {
        PSP35EnumerableStorage::get_mut(self)
    }

    default fn _total_balance_of_id(&self, id: &Id) -> Balance {
        self._enumerable().total_balance.get(id).unwrap_or(0)
    }
}

impl<T: PSP35EnumerableStorage + Flush> PSP35Enumerable for T {
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP35Error> {
        self._enumerable()
            .enumerable
            .get_value(&Some(&owner), &index)
            .ok_or(PSP35Error::TokenNotExists)
    }

    default fn token_by_index(&self, index: u128) -> Result<Id, PSP35Error> {
        self._enumerable()
            .enumerable
            .get_value(&None, &index)
            .ok_or(PSP35Error::TokenNotExists)
    }
}
