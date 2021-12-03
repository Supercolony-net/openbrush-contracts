/// Extension for `PSP721` that adds enumerability of all the token ids in the contract
/// as well as all token ids owned by each account as defined in the EIP
/// https://docs.openzeppelin.com/contracts/4.x/api/token/erc721#IERC721Enumerable
use crate::traits::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        InkStorage,
    },
};
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use psp721_derive::PSP721EnumerableStorage;

use crate::traits::Id;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP721EnumerableData {
    pub owned_tokens: StorageHashMap<AccountId, Vec<Id>>,
    pub owned_tokens_index: StorageHashMap<Id, u32>,
    pub all_tokens: Vec<Id>,
    pub all_tokens_index: StorageHashMap<Id, u32>,
}

declare_storage_trait!(PSP721EnumerableStorage, PSP721EnumerableData);

#[brush::wrapper]
pub type PSP721EnumerableWrapper = dyn PSP721Enumerable + PSP721;

#[brush::trait_definition]
pub trait PSP721Enumerable: PSP721EnumerableStorage + PSP721 {
    /// Returns the id of token owned by `owner` at a given `index` of its token list
    #[ink(message)]
    fn token_of_owner_by_index(&self, owner: AccountId, index: u32) -> Result<Id, PSP721Error> {
        if index >= self.balance_of(owner) {
            return Err(PSP721Error::Custom(String::from("Owner index out of bounds")))
        }
        // at this point there exists a vector for user so we just unwrap
        Ok(PSP721EnumerableStorage::get(self).owned_tokens.get(&owner).unwrap()[index as usize])
    }

    /// Returns the total amount of tokens in the contract
    #[ink(message)]
    fn total_supply(&self) -> u32 {
        PSP721EnumerableStorage::get(self).all_tokens.len() as u32
    }

    /// Returns the id of token at a given `index` of all the tokens in the contract
    #[ink(message)]
    fn token_by_index(&self, index: u32) -> Result<Id, PSP721Error> {
        if index >= self.total_supply() {
            return Err(PSP721Error::Custom(String::from("Global index out of bounds")))
        }
        Ok(PSP721EnumerableStorage::get(self).all_tokens[index as usize])
    }

    /// Call this method in your `_before_token_transfer`
    ///
    /// When `from` and `to` are both non-zero, ``from``'s `tokenId` will be transferred to `to`
    /// When `from` is zero, `tokenId` will be minted for `to`
    /// When `to` is zero, ``from``'s `tokenId` will be burned
    /// `from` cannot be the zero address
    /// `to` cannot be the zero address.
    fn handle_token_transfer(&mut self, from: &AccountId, to: &AccountId, id: &Id) {
        if from.is_zero() {
            self.add_token_to_all_tokens_enumeration(*id);
        } else {
            self.remove_token_from_owner_enumeration(*from, *id);
        }

        if to.is_zero() {
            self.remove_token_from_all_tokens_enumeration(*id);
        } else {
            self.add_token_to_owner_enumeration(*to, *id);
        }
    }

    /// Helper function to add a token to ownership tracking data structures
    /// `to` is the new token owner
    /// `token_id` is the id of the new token
    fn add_token_to_owner_enumeration(&mut self, to: AccountId, token_id: Id) {
        let length = self.balance_of(to);
        let storage = PSP721EnumerableStorage::get_mut(self);
        storage
            .owned_tokens
            .entry(to)
            .and_modify(|vec| vec.push(token_id))
            .or_insert_with(|| {
                let mut vec = Vec::<Id>::new();
                vec.push(token_id);
                vec
            });
        storage.owned_tokens_index.insert(token_id, length);
    }

    /// Helper function to add a token to token tracking data structures
    /// `token_id` is the id of the new token
    fn add_token_to_all_tokens_enumeration(&mut self, token_id: Id) {
        let len = self.total_supply();
        let storage = PSP721EnumerableStorage::get_mut(self);
        storage.all_tokens_index.insert(token_id, len);
        storage.all_tokens.push(token_id);
    }

    /// Helper function to remove a token from contract ownership-tracking data structures.  
    /// `from` is the owner of the token
    /// `token_id` is the id the token to be removed from the tokens list of the given address
    fn remove_token_from_owner_enumeration(&mut self, from: AccountId, token_id: Id) {
        let last_index = self.balance_of(from) - 1;
        let storage = PSP721EnumerableStorage::get_mut(self);
        let token_index = *storage.owned_tokens_index.get(&token_id).unwrap();

        if token_index != last_index {
            let last_id = storage.owned_tokens.get(&from).unwrap()[last_index as usize];

            storage
                .owned_tokens
                .entry(from)
                .and_modify(|vec| vec[token_index as usize] = last_id);
            storage.owned_tokens_index.insert(last_id, token_index);
        }

        storage.owned_tokens_index.take(&token_id);
        storage.owned_tokens.entry(from).and_modify(|vec| {
            vec.pop();
            ()
        });
    }

    /// Helper function to remove a token from contract token tracking data structures
    /// `token_id` is the id of the token to be removed from the tokens list
    fn remove_token_from_all_tokens_enumeration(&mut self, token_id: Id) {
        let last_token_index = self.total_supply() - 1;
        let storage = PSP721EnumerableStorage::get_mut(self);
        let token_index = *storage.all_tokens_index.get(&token_id).unwrap();

        let last_token_id = storage.all_tokens[last_token_index as usize];

        storage.all_tokens[token_index as usize] = last_token_id;
        storage.all_tokens_index.insert(last_token_id, token_index);

        storage.all_tokens_index.take(&token_id);
        storage.all_tokens.pop();
    }
}
