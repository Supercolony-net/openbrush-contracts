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

use super::{
    Helper,
    TypeGuard,
    ValueGuard,
};
use core::marker::PhantomData;
use ink_storage::traits::{
    ExtKeyPtr,
    KeyPtr,
    PackedLayout,
    SpreadAllocate,
    SpreadLayout,
};

use crate::storage::RefGuard;
use ink_primitives::Key;

/// It is a more restricted version of the `Mapping` from ink!. That mapping can be used to unify
/// the API calls to the `Mapping` to avoid monomorphization to reduce the size of contracts.
/// It verifies that all calls are done with the same type during compilation.
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Mapping<K, V, TGK = RefGuard<K>, TGV = ValueGuard<V>> {
    offset_key: Key,
    _marker: PhantomData<fn() -> (K, V, TGK, TGV)>,
}

/// We implement this manually because the derived implementation adds trait bounds.
impl<K, V, TGK, TGV> Default for Mapping<K, V, TGK, TGV> {
    fn default() -> Self {
        Self {
            offset_key: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<K, V, TGK, TGV> core::fmt::Debug for Mapping<K, V, TGK, TGV> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mapping").field("offset_key", &self.offset_key).finish()
    }
}

impl<K, V, TGK, TGV> Mapping<K, V, TGK, TGV> {
    /// Creates a new empty `Mapping`.
    fn new(offset_key: Key) -> Self {
        Self {
            offset_key,
            _marker: Default::default(),
        }
    }
}

impl<K, V, TGK, TGV> Mapping<K, V, TGK, TGV>
where
    K: PackedLayout,
    V: PackedLayout,
{
    /// Insert the given `value` to the contract storage.
    #[inline]
    pub fn insert<'a, 'b>(&mut self, key: TGK::Type, value: &TGV::Type)
    where
        TGK: TypeGuard<'a>,
        TGV: TypeGuard<'b>,
        TGK::Type: scale::Encode,
        TGV::Type: PackedLayout,
    {
        Helper::<TGK::Type, TGV::Type, &Key>::new(&self.offset_key).insert(key, value)
    }

    /// Insert the given `value` to the contract storage.
    ///
    /// Returns the size of the pre-existing value at the specified key if any.
    #[inline]
    pub fn insert_return_size<'a, 'b>(&mut self, key: TGK::Type, value: &TGV::Type) -> Option<u32>
    where
        TGK: TypeGuard<'a>,
        TGV: TypeGuard<'b>,
        TGK::Type: scale::Encode,
        TGV::Type: PackedLayout,
    {
        Helper::<TGK::Type, TGV::Type, &Key>::new(&self.offset_key).insert_return_size(key, value)
    }

    /// Get the `value` at `key` from the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline]
    pub fn get<'a>(&self, key: TGK::Type) -> Option<V>
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        Helper::<TGK::Type, V, &Key>::new(&self.offset_key).get(key)
    }

    /// Get the size of a value stored at `key` in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline]
    pub fn size<'a>(&self, key: TGK::Type) -> Option<u32>
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        Helper::<TGK::Type, (), &Key>::new(&self.offset_key).size(key)
    }

    /// Checks if a value is stored at the given `key` in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline]
    pub fn contains<'a>(&self, key: TGK::Type) -> bool
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        Helper::<TGK::Type, (), &Key>::new(&self.offset_key).contains(key)
    }

    /// Clears the value at `key` from storage.
    pub fn remove<'a>(&self, key: TGK::Type)
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        Helper::<TGK::Type, V, &Key>::new(&self.offset_key).remove(key)
    }
}

impl<K, V, TGK, TGV> SpreadLayout for Mapping<K, V, TGK, TGV> {
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = false;

    #[inline(always)]
    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        // Note: There is no need to pull anything from the storage for the
        //       mapping type since it initializes itself entirely by the
        //       given key pointer.
        Self::new(*ExtKeyPtr::next_for::<Self>(ptr))
    }

    #[inline(always)]
    fn push_spread(&self, ptr: &mut KeyPtr) {
        // Note: The mapping type does not store any state in its associated
        //       storage region, therefore only the pointer has to be incremented.
        ptr.advance_by(Self::FOOTPRINT);
    }

    #[inline(always)]
    fn clear_spread(&self, ptr: &mut KeyPtr) {
        // Note: The mapping type is not aware of its elements, therefore
        //       it is not possible to clean up after itself.
        ptr.advance_by(Self::FOOTPRINT);
    }
}

impl<K, V, TGK, TGV> SpreadAllocate for Mapping<K, V, TGK, TGV> {
    #[inline(always)]
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        // Note: The mapping type initializes itself entirely by the key pointer.
        Self::new(*ExtKeyPtr::next_for::<Self>(ptr))
    }
}

#[cfg(feature = "std")]
const _: () = {
    use ink_metadata::layout::{
        CellLayout,
        Layout,
        LayoutKey,
    };
    use ink_storage::traits::StorageLayout;

    impl<K, V, TGK, TGV> StorageLayout for Mapping<K, V, TGK, TGV>
    where
        K: scale_info::TypeInfo + 'static,
        V: scale_info::TypeInfo + 'static,
    {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            Layout::Cell(CellLayout::new::<ink_storage::Mapping<K, V>>(LayoutKey::from(
                key_ptr.advance_by(1),
            )))
        }
    }
};
