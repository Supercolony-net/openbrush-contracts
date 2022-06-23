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

use super::{
    RefGuard,
    TypeGuard,
    ValueGuard,
};
use crate::storage::RawMapping;
use core::marker::PhantomData;
use ink_primitives::{
    Key,
    KeyPtr,
};
use ink_storage::traits::{
    ExtKeyPtr,
    PackedLayout,
    SpreadAllocate,
    SpreadLayout,
};

// TODO: More doc
/// A mapping of one key to many values. The mapping provides iteration functionality over all
/// key's values.
pub struct MultiMapping<K, V, TGK = RefGuard<K>, TGV = ValueGuard<V>> {
    offset_key: Key,
    _marker: PhantomData<fn() -> (K, V, TGK, TGV)>,
}

type ValueToIndex<'a, TGK, TGV> = &'a (<TGK as TypeGuard<'a>>::Type, &'a <TGV as TypeGuard<'a>>::Type);
type IndexToValue<'a, TGK> = &'a (<TGK as TypeGuard<'a>>::Type, &'a u128);

impl<K, V, TGK, TGV> MultiMapping<K, V, TGK, TGV> {
    fn new(offset_key: Key) -> Self {
        Self {
            offset_key,
            _marker: Default::default(),
        }
    }

    // Contains count of values by key.
    // key_count: Mapping<K, u128>,
    fn key_count(&self) -> RawMapping<<TGK as TypeGuard>::Type, u128, (&Key, &u32)>
    where
        for<'a> TGK: TypeGuard<'a>,
    {
        RawMapping::new((&self.offset_key, &0))
    }

    // Mapping from key's value to local index.
    // value_to_index: Mapping<(K, V), u128>,
    fn value_to_index(&self) -> RawMapping<ValueToIndex<TGK, TGV>, u128, (&Key, &u32)>
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
    {
        RawMapping::new((&self.offset_key, &1))
    }

    // Mapping from local key's index to value.
    // index_to_value: Mapping<(K, u128), V>,
    fn index_to_value(&self) -> RawMapping<IndexToValue<TGK>, <TGV as TypeGuard>::Type, (&Key, &u32)>
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
    {
        RawMapping::new((&self.offset_key, &2))
    }
}

impl<K, V, TGK, TGV> Default for MultiMapping<K, V, TGK, TGV> {
    fn default() -> Self {
        Self {
            offset_key: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<K, V, TGK, TGV> core::fmt::Debug for MultiMapping<K, V, TGK, TGV> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultiMapping")
            .field("offset_key", &self.offset_key)
            .finish()
    }
}

impl<K, V, TGK, TGV> MultiMapping<K, V, TGK, TGV>
where
    K: PackedLayout,
    V: PackedLayout,
{
    /// Insert the given `value` to the contract storage at `key`.
    pub fn insert<'b>(&'b mut self, key: <TGK as TypeGuard<'b>>::Type, value: &<TGV as TypeGuard<'b>>::Type)
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout,
    {
        let index: u128 = match self.get_index(key, value) {
            None => {
                let count = self.count(key);
                self.key_count().insert(key, &(count + 1));
                count
            }
            Some(index) => index,
        };
        self.value_to_index().insert(&(key, value), &index);
        let size = self.index_to_value().insert(&(key, &index), value);
        size
    }

    // /// Insert the given `value` to the contract storage at `key`.
    // ///
    // /// Returns the size of the pre-existing value at the specified key if any.
    // pub fn insert_return_size<'b>(
    //     &'b mut self,
    //     key: <TGK as TypeGuard<'b>>::Type,
    //     value: &<TGV as TypeGuard<'b>>::Type,
    // ) -> Option<u32>
    // where
    //     for<'a> TGK: TypeGuard<'a>,
    //     for<'a> TGV: TypeGuard<'a>,
    //     for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
    //     for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout,
    // {
    //     let index: u128 = match self.get_index(key, value) {
    //         None => {
    //             let count = self.count(key);
    //             self.key_count().insert(key, &(count + 1));
    //             count
    //         }
    //         Some(index) => index,
    //     };
    //     self.value_to_index().insert_return_size(&(key, value), &index);
    //     let size = self.index_to_value().insert_return_size(&(key, &index), value);
    //     size
    // }

    /// Returns the count of values stored under the `key`.
    #[inline]
    pub fn count<'b>(&'b self, key: <TGK as TypeGuard<'b>>::Type) -> u128
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode,
    {
        self.key_count().get(key).unwrap_or_default()
    }

    /// Get the `value` at (`key`, `index`) from the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given (`key`, `index`).
    #[inline]
    pub fn get_value<'b>(&self, key: <TGK as TypeGuard<'b>>::Type, index: &u128) -> Option<V>
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
    {
        RawMapping::<IndexToValue<TGK>, V, _>::new((&self.offset_key, &2)).get(&(key, index))
    }

    /// Get the `index` of (`key`, `value`) from the contract storage.
    ///
    /// Returns `None` if no `value` exists for the given `key`.
    #[inline]
    pub fn get_index<'b>(&self, key: <TGK as TypeGuard<'b>>::Type, value: &<TGV as TypeGuard<'b>>::Type) -> Option<u128>
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout,
    {
        RawMapping::<ValueToIndex<TGK, TGV>, u128, _>::new((&self.offset_key, &1)).get(&(key, value))
    }

    /// Get the size of a value stored at (`key`, `value`) in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given (`key`, `value`).
    #[inline]
    pub fn size_value<'b>(
        &'b self,
        key: <TGK as TypeGuard<'b>>::Type,
        value: &<TGV as TypeGuard<'b>>::Type,
    ) -> Option<u32>
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout,
    {
        if let Some(index) = self.get_index(key, value) {
            self.index_to_value().size(&(key, &index))
        } else {
            None
        }
    }

    /// Get the size of a value stored at (`key`, `index`) in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given (`key`, `index`).
    #[inline]
    pub fn size_index<'b>(&'b self, key: <TGK as TypeGuard<'b>>::Type, index: &u128) -> Option<u32>
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode,
    {
        self.index_to_value().size(&(key, index))
    }

    /// Checks if any value is stored at the given `key` in the contract storage.
    #[inline]
    pub fn contains<'b>(&'b self, key: <TGK as TypeGuard<'b>>::Type) -> bool
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode,
    {
        self.count(key) > 0
    }

    /// Checks if the `value` is stored at the given `key` in the contract storage.
    #[inline]
    pub fn contains_value<'b>(&'b self, key: <TGK as TypeGuard<'b>>::Type, value: &<TGV as TypeGuard<'b>>::Type) -> bool
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout,
    {
        self.value_to_index().contains(&(key, value))
    }

    /// Checks if any value is stored at the given (`key`, `index`) in the contract storage.
    #[inline]
    pub fn contains_index<'b>(&'b self, key: <TGK as TypeGuard<'b>>::Type, index: &u128) -> bool
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode,
    {
        self.index_to_value().contains(&(key, index))
    }

    /// Clears the `value` at `key` from storage.
    pub fn remove_value<'b>(&'b mut self, key: <TGK as TypeGuard<'b>>::Type, value: &<TGV as TypeGuard<'b>>::Type)
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout + From<V>,
    {
        let op_index = self.get_index(key, value);

        let index;
        if let Some(op_index) = op_index {
            index = op_index;
        } else {
            return
        }
        let index = &index;
        self.swap_and_remove(key, value, index);
    }

    /// Clears the value at (`key`, `index`) from storage.
    pub fn remove_index<'b>(&'b mut self, key: <TGK as TypeGuard<'b>>::Type, index: &u128)
    where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout + From<V>,
    {
        let op_value = self.get_value(key, index);

        let value;
        if let Some(op_value) = op_value {
            value = op_value;
        } else {
            return
        }
        self.swap_and_remove(key, &value.into(), index);
    }

    fn swap_and_remove<'b>(
        &'b mut self,
        key: <TGK as TypeGuard<'b>>::Type,
        value: &<TGV as TypeGuard<'b>>::Type,
        index: &u128,
    ) where
        for<'a> TGK: TypeGuard<'a>,
        for<'a> TGV: TypeGuard<'a>,
        for<'a> <TGK as TypeGuard<'a>>::Type: scale::Encode + Copy,
        for<'a> <TGV as TypeGuard<'a>>::Type: PackedLayout + From<V>,
    {
        let last_index = &(self.count(key) - 1);

        if last_index != index {
            let last_value = &self
                .get_value(key, last_index)
                .expect("The value under the last index should exist")
                .into();
            self.index_to_value().insert(&(key, index), last_value);
            self.value_to_index().insert(&(key, last_value), &index);
        }

        self.index_to_value().remove(&(key, last_index));
        self.value_to_index().remove(&(key, value));
        self.key_count().insert(key, last_index);
    }
}

impl<K, V, TGK, TGV> SpreadLayout for MultiMapping<K, V, TGK, TGV> {
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

impl<K, V, TGK, TGV> SpreadAllocate for MultiMapping<K, V, TGK, TGV> {
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
    use scale_info::{
        build::Fields,
        type_params,
        Path,
        Type,
        TypeInfo,
    };

    impl<K, V, TGK, TGV> TypeInfo for MultiMapping<K, V, TGK, TGV>
    where
        K: TypeInfo + 'static,
        V: TypeInfo + 'static,
        TGK: 'static,
        TGV: 'static,
    {
        type Identity = Self;

        fn type_info() -> Type {
            Type::builder()
                .path(Path::new("MultiMapping", module_path!()))
                .type_params(type_params![K, V])
                .composite(Fields::unnamed().field(|f| f.ty::<[(K, V)]>()))
        }
    }

    impl<K, V, TGK, TGV> StorageLayout for MultiMapping<K, V, TGK, TGV>
    where
        K: scale_info::TypeInfo + 'static,
        V: scale_info::TypeInfo + 'static,
        TGK: 'static,
        TGV: 'static,
    {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            Layout::Cell(CellLayout::new::<Self>(LayoutKey::from(key_ptr.advance_by(1))))
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[ink_lang::test]
    fn insert_and_count_works() {
        let mut mapping: MultiMapping<u128, u128> = MultiMapping::default();
        mapping.insert(&1, &1);
        mapping.insert(&1, &2);
        assert_eq!(mapping.count(&1), 2);
    }

    #[ink_lang::test]
    fn double_insert_and_count_works() {
        let mut mapping: MultiMapping<u128, u128> = MultiMapping::default();
        mapping.insert(&1, &1);
        mapping.insert(&1, &1);
        assert_eq!(mapping.count(&1), 1);
    }

    #[ink_lang::test]
    fn get_works() {
        let mut mapping: MultiMapping<u128, u128> = MultiMapping::default();
        mapping.insert(&1, &1);
        assert_eq!(mapping.get_index(&1, &1), Some(0));
        assert_eq!(mapping.get_value(&1, &0), Some(1));

        mapping.insert(&1, &2);
        assert_eq!(mapping.get_index(&1, &1), Some(0));
        assert_eq!(mapping.get_index(&1, &2), Some(1));

        assert_eq!(mapping.get_value(&1, &0), Some(1));
        assert_eq!(mapping.get_value(&1, &1), Some(2));
    }

    #[ink_lang::test]
    fn remove_works() {
        let mut mapping: MultiMapping<u128, u128> = MultiMapping::default();
        mapping.insert(&1, &1);
        mapping.insert(&1, &2);
        assert_eq!(mapping.get_index(&1, &1), Some(0));
        assert_eq!(mapping.get_index(&1, &2), Some(1));

        assert_eq!(mapping.count(&1), 2);

        mapping.remove_value(&1, &1);
        assert_eq!(mapping.count(&1), 1);
        assert_eq!(mapping.get_value(&1, &0), Some(2));

        mapping.insert(&1, &1);

        assert_eq!(mapping.count(&1), 2);
        assert_eq!(mapping.get_value(&1, &0), Some(2));
        assert_eq!(mapping.get_value(&1, &1), Some(1));

        mapping.remove_index(&1, &0);

        assert_eq!(mapping.count(&1), 1);
        assert_eq!(mapping.get_value(&1, &0), Some(1));
    }

    #[ink_lang::test]
    fn remove_non_exist_works() {
        let mut mapping: MultiMapping<u128, u128> = MultiMapping::default();
        mapping.insert(&1, &1);
        mapping.insert(&1, &2);
        mapping.insert(&1, &3);
        assert_eq!(mapping.count(&1), 3);

        mapping.remove_value(&1, &4);
        assert_eq!(mapping.count(&1), 3);

        mapping.remove_value(&1, &2);
        assert_eq!(mapping.count(&1), 2);
        mapping.remove_value(&1, &2);
        assert_eq!(mapping.count(&1), 2);
    }

    #[ink_lang::test]
    fn contain_works() {
        let mut mapping: MultiMapping<u128, u128> = MultiMapping::default();

        mapping.insert(&1, &1);
        mapping.insert(&1, &2);

        assert_eq!(mapping.contains(&1), true);
        assert_eq!(mapping.contains(&2), false);
        assert_eq!(mapping.contains_index(&1, &1), true);
        assert_eq!(mapping.contains_index(&1, &2), false);
        assert_eq!(mapping.contains_value(&1, &1), true);
        assert_eq!(mapping.contains_value(&1, &3), false);
    }
}
