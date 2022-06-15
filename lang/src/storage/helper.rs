// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A simple Helper to contract storage.
//!
//! # Note
//!
//! This Helper doesn't actually "own" any data.
//! Instead it is just a simple wrapper around the contract storage facilities.

use core::marker::PhantomData;
use ink_storage::traits::{
    push_packed_root,
    PackedLayout,
    SpreadLayout,
};

use ink_env::hash::{
    Blake2x256,
    HashOutput,
};
use ink_primitives::Key;

pub struct Helper<K, V, T = Key> {
    prefix: T,
    _marker: PhantomData<fn() -> (K, V)>,
}

// TODO: Doc
impl<K, V, T> Helper<K, V, T> {
    /// Creates a new empty `Helper`.
    #[inline(always)]
    pub fn new(prefix: T) -> Self {
        Self {
            prefix,
            _marker: Default::default(),
        }
    }
}

impl<K, V, T> Helper<K, V, T>
where
    T: scale::Encode + Copy,
{
    /// Insert the given `value` to the contract storage.
    #[inline(always)]
    pub fn insert(&self, key: K, value: &V)
    where
        K: scale::Encode,
        V: PackedLayout,
    {
        self.insert_return_size(key, value);
    }

    /// Insert the given `value` to the contract storage.
    ///
    /// Returns the size of the pre-existing value at the specified key if any.
    #[inline(always)]
    pub fn insert_return_size(&self, key: K, value: &V) -> Option<u32>
    where
        K: scale::Encode,
        V: PackedLayout,
    {
        push_packed_root(value, &self.storage_key(key))
    }

    /// Get the `value` at `key` from the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline(always)]
    pub fn get(&self, key: K) -> Option<V>
    where
        K: scale::Encode,
        V: PackedLayout,
    {
        self.get_contract_storage(&self.storage_key(key))
    }

    /// Get the size of a value stored at `key` in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline(always)]
    pub fn size(&self, key: K) -> Option<u32>
    where
        K: scale::Encode,
    {
        ink_env::contract_storage_contains(&self.storage_key(key))
    }

    /// Checks if a value is stored at the given `key` in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline(always)]
    pub fn contains(&self, key: K) -> bool
    where
        K: scale::Encode,
    {
        ink_env::contract_storage_contains(&self.storage_key(key)).is_some()
    }

    /// Clears the value at `key` from storage.
    #[inline(always)]
    pub fn remove(&self, key: K)
    where
        K: scale::Encode,
        V: PackedLayout,
    {
        let storage_key = self.storage_key(key);
        if <V as SpreadLayout>::REQUIRES_DEEP_CLEAN_UP {
            // There are types which need to perform some action before being cleared. Here we
            // indicate to those types that they should start tidying up.
            if let Some(value) = self.get_contract_storage(&storage_key) {
                <V as PackedLayout>::clear_packed(&value, &storage_key);
            }
        }
        ink_env::clear_contract_storage(&storage_key);
    }

    /// Returns a `Key` pointer used internally by the storage API.
    ///
    /// This key is a combination of the `Helper`'s internal `offset_key`
    /// and the user provided `key`.
    #[inline(always)]
    fn storage_key(&self, key: K) -> Key
    where
        K: scale::Encode,
    {
        let encodedable_key = (self.prefix, key);
        Self::storage_key_inline(&encodedable_key)
    }

    #[inline(never)]
    fn storage_key_inline<E>(key: &E) -> Key
    where
        E: scale::Encode,
    {
        let mut output = <Blake2x256 as HashOutput>::Type::default();
        ink_env::hash_encoded::<Blake2x256, _>(key, &mut output);
        output.into()
    }

    fn get_contract_storage(&self, key: &Key) -> Option<V>
    where
        K: scale::Encode,
        V: PackedLayout,
    {
        ink_env::get_contract_storage::<V>(key)
            .unwrap_or_else(|error| panic!("failed to pull packed from root key {}: {:?}", key, error))
            .map(|mut value| {
                // In case the contract storage is occupied at the root key
                // we handle the Option<T> as if it was a T.
                <V as PackedLayout>::pull_packed(&mut value, key);
                value
            })
    }
}
