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

pub struct RawMapping<K, V, T = Key> {
    prefix: T,
    _marker: PhantomData<fn() -> (K, V)>,
}

/// It is the implementation of `Mapping` functionality without storing it as a storage field.
/// It can be used to store value under the key manually.
impl<K, V, T> RawMapping<K, V, T> {
    /// Creates a new empty `RawMapping`.
    #[inline(always)]
    pub fn new(prefix: T) -> Self {
        Self {
            prefix,
            _marker: Default::default(),
        }
    }
}

impl<K, V, T> RawMapping<K, V, T>
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
        push_packed_root(value, &self.storage_key(key))
    }

    // /// Insert the given `value` to the contract storage.
    // ///
    // /// Returns the size of the pre-existing value at the specified key if any.
    // #[inline(always)]
    // pub fn insert_return_size(&self, key: K, value: &V) -> Option<u32>
    // where
    //     K: scale::Encode,
    //     V: PackedLayout,
    // {
    //     push_packed_root(value, &self.storage_key(key))
    // }

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
    /// This key is a combination of the `RawMapping`'s internal `offset_key`
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
