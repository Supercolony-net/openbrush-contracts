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
use ink_storage::Mapping;
use openbrush::traits::{
    AccountId,
    Balance,
    Flush,
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP35EnumerableData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP35EnumerableData {
    pub enumerable: EnumerableMapping,
    pub amount_of_tokens: Mapping<Option<AccountId>, u128>,
    pub _reserved: Option<()>,
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

    fn _amount_of_tokens(&self, owner: &Option<AccountId>) -> u128;

    fn _increase_token_balance(
        &mut self,
        owner: &Option<AccountId>,
        id: &Id,
        amount: &Balance,
    ) -> Result<(), PSP35Error>;

    fn _decrease_token_balance(
        &mut self,
        owner: &Option<AccountId>,
        id: &Id,
        amount: &Balance,
    ) -> Result<(), PSP35Error>;
}

impl<T: PSP35EnumerableStorage + Flush> PSP35EnumerableInternal for T {
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
            self._increase_token_balance(&None, id, amount)?;
        } else {
            let from = from.unwrap();
            self._decrease_token_balance(&Some(from.clone()), id, amount)?;
        }

        if to.is_none() {
            self._decrease_token_balance(&None, id, amount)?;
        } else {
            let to = to.unwrap();
            self._increase_token_balance(&Some(to.clone()), id, amount)?;
        }

        Ok(())
    }

    fn _amount_of_tokens(&self, owner: &Option<AccountId>) -> u128 {
        PSP35EnumerableStorage::get(self)
            .amount_of_tokens
            .get(owner)
            .unwrap_or(0)
    }

    fn _increase_token_balance(
        &mut self,
        owner: &Option<AccountId>,
        id: &Id,
        amount: &Balance,
    ) -> Result<(), PSP35Error> {
        let index = PSP35EnumerableStorage::get(self).enumerable.get_index_by_id(owner, &id);

        if index.is_some() {
            let index = index.unwrap();
            let initial_balance = PSP35EnumerableStorage::get(self)
                .enumerable
                .get_balance_by_index(owner, &index)?;

            PSP35EnumerableStorage::get_mut(self)
                .enumerable
                .insert(owner, &id, &(initial_balance + amount), &index);
        } else {
            let last_free_index = self._amount_of_tokens(owner);
            PSP35EnumerableStorage::get_mut(self)
                .enumerable
                .insert(owner, &id, &amount, &last_free_index);

            PSP35EnumerableStorage::get_mut(self)
                .amount_of_tokens
                .insert(owner, &(last_free_index + 1));
        }

        Ok(())
    }

    fn _decrease_token_balance(
        &mut self,
        owner: &Option<AccountId>,
        id: &Id,
        amount: &Balance,
    ) -> Result<(), PSP35Error> {
        let index = PSP35EnumerableStorage::get(self).enumerable.get_index_by_id(&owner, id);

        if index.is_none() {
            return Err(PSP35Error::TokenNotExists)
        }

        let index = index.unwrap();

        let initial_balance = PSP35EnumerableStorage::get(self)
            .enumerable
            .get_balance_by_index(owner, &index)?;

        if initial_balance < *amount {
            return Err(PSP35Error::InsufficientBalance)
        }

        if initial_balance > *amount {
            let initial_balance = PSP35EnumerableStorage::get(self)
                .enumerable
                .get_balance_by_index(owner, &index)?;

            PSP35EnumerableStorage::get_mut(self)
                .enumerable
                .insert(owner, &id, &(initial_balance - amount), &index);
        } else {
            let last_index = self._amount_of_tokens(&owner) - 1;
            PSP35EnumerableStorage::get_mut(self)
                .enumerable
                .remove(owner, &id, &last_index)?;

            PSP35EnumerableStorage::get_mut(self)
                .amount_of_tokens
                .insert(&owner, &last_index);
        }

        Ok(())
    }
}

impl<T: PSP35EnumerableStorage + Flush> PSP35Enumerable for T {
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP35Error> {
        PSP35EnumerableStorage::get(self)
            .enumerable
            .get_id_by_index(&Some(owner), &index)
    }

    default fn token_by_index(&self, index: u128) -> Result<Id, PSP35Error> {
        PSP35EnumerableStorage::get(self)
            .enumerable
            .get_id_by_index(&None, &index)
    }
}

#[derive(Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate)]
#[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
pub struct EnumerableMapping {
    /// Mapping from index to `Id`.
    ///
    /// ** Note ** Owner can be `None` to track existence of the token in the contract
    id_to_index: Mapping<(Option<AccountId>, Id), u128>,
    /// Mapping from owner's index to `Id`.
    ///
    /// ** Note ** Owner can be `None` that means it is a contract.
    index_to_id: Mapping<(Option<AccountId>, u128), (Id, Balance)>,
}

impl EnumerableMapping {
    pub fn insert(&mut self, owner: &Option<AccountId>, id: &Id, amount: &Balance, index: &u128) {
        self.id_to_index.insert((owner, id), index);
        self.index_to_id.insert((owner, index), &(id.clone(), amount.clone()));
    }

    pub fn remove(&mut self, owner: &Option<AccountId>, id: &Id, last_index: &u128) -> Result<(), PSP35Error> {
        let index = self.id_to_index.get((owner, id)).ok_or(PSP35Error::TokenNotExists)?;

        if last_index != &index {
            let (last_id, balance) = self.get_by_index(owner, last_index)?;
            self.index_to_id.insert((owner, &index), &(last_id.clone(), balance));
            self.id_to_index.insert((owner, &last_id), &index);
        }

        self.index_to_id.remove((owner, &last_index));
        self.id_to_index.remove((owner, id));

        Ok(())
    }

    pub fn get_by_index(&self, owner: &Option<AccountId>, index: &u128) -> Result<(Id, Balance), PSP35Error> {
        self.index_to_id.get((owner, index)).ok_or(PSP35Error::TokenNotExists)
    }

    pub fn get_id_by_index(&self, owner: &Option<AccountId>, index: &u128) -> Result<Id, PSP35Error> {
        let id = self
            .index_to_id
            .get((owner, index))
            .ok_or(PSP35Error::TokenNotExists)?
            .0;
        Ok(id)
    }

    pub fn get_balance_by_index(&self, owner: &Option<AccountId>, index: &u128) -> Result<Balance, PSP35Error> {
        let balance = self
            .index_to_id
            .get((owner, index))
            .ok_or(PSP35Error::TokenNotExists)?
            .1;
        Ok(balance)
    }

    pub fn get_index_by_id(&self, owner: &Option<AccountId>, id: &Id) -> Option<u128> {
        self.id_to_index.get((owner, id))
    }
}
