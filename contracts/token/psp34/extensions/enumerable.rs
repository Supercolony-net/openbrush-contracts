pub use crate::{
    psp34::*,
    traits::psp34::extensions::enumerable::*,
};
use brush::traits::{
    AccountId,
    Flush,
};
pub use derive::PSP34EnumerableStorage;
use ink_storage::Mapping;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP34EnumerableData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP34EnumerableData {
    pub enumerable: EnumerableMapping,
    pub _reserved: Option<()>,
}

pub trait PSP34EnumerableStorage: PSP34Storage + ::brush::traits::InkStorage {
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
}

impl<T: PSP34EnumerableStorage + Flush> PSP34EnumerableInternal for T {
    default fn _track_id_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error> {
        if from.is_none() {
            let last_free_index = self._total_supply();
            PSP34EnumerableStorage::get_mut(self)
                .enumerable
                .insert(&None, id, &last_free_index);
        } else {
            let from = from.unwrap();
            let last_index = (self._balance_of(from) - 1) as u128;
            PSP34EnumerableStorage::get_mut(self)
                .enumerable
                .remove(&Some(from.clone()), id, &last_index)?;
        }

        if to.is_none() {
            let last_index = self._total_supply() - 1;
            PSP34EnumerableStorage::get_mut(self)
                .enumerable
                .remove(&None, id, &last_index)?;
        } else {
            let to = to.unwrap();
            let last_free_index = (self._balance_of(to)) as u128;
            PSP34EnumerableStorage::get_mut(self)
                .enumerable
                .insert(&Some(to.clone()), id, &last_free_index);
        }

        Ok(())
    }
}

impl<T: PSP34EnumerableStorage + Flush> PSP34Enumerable for T {
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        PSP34EnumerableStorage::get(self)
            .enumerable
            .get_by_index(&Some(owner), &index)
    }

    default fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        PSP34EnumerableStorage::get(self).enumerable.get_by_index(&None, &index)
    }
}

#[derive(Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate)]
#[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
pub struct EnumerableMapping {
    /// Mapping from index to `Id`.
    ///
    /// ** Note ** Owner can be `None` that means it is a contract.
    id_to_index: Mapping<(Option<AccountId>, Id), u128>,
    /// Mapping from owner's index to `Id`.
    ///
    /// ** Note ** Owner can be `None` that means it is a contract.
    index_to_id: Mapping<(Option<AccountId>, u128), Id>,
}

impl EnumerableMapping {
    pub fn insert(&mut self, owner: &Option<AccountId>, id: &Id, index: &u128) {
        self.id_to_index.insert((owner, id), index);
        self.index_to_id.insert((owner, index), id);
    }

    pub fn remove(&mut self, owner: &Option<AccountId>, id: &Id, last_index: &u128) -> Result<(), PSP34Error> {
        let index = self.id_to_index.get((owner, id)).ok_or(PSP34Error::TokenNotExists)?;

        if last_index != &index {
            let last_id = self
                .index_to_id
                .get((owner, last_index))
                .ok_or(PSP34Error::TokenNotExists)?;
            self.index_to_id.insert((owner, &index), &last_id);
            self.id_to_index.insert((owner, &last_id), &index);
        }

        self.index_to_id.remove((owner, &last_index));
        self.id_to_index.remove((owner, id));

        Ok(())
    }

    pub fn get_by_index(&self, owner: &Option<AccountId>, index: &u128) -> Result<Id, PSP34Error> {
        self.index_to_id.get((owner, index)).ok_or(PSP34Error::TokenNotExists)
    }
}
