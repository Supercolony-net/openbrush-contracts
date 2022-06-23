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
    RawMapping,
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
        RawMapping::<TGK::Type, TGV::Type, &Key>::new(&self.offset_key).insert(key, value)
    }

    // /// Insert the given `value` to the contract storage.
    // ///
    // /// Returns the size of the pre-existing value at the specified key if any.
    // #[inline]
    // pub fn insert_return_size<'a, 'b>(&mut self, key: TGK::Type, value: &TGV::Type) -> Option<u32>
    // where
    //     TGK: TypeGuard<'a>,
    //     TGV: TypeGuard<'b>,
    //     TGK::Type: scale::Encode,
    //     TGV::Type: PackedLayout,
    // {
    //     RawMapping::<TGK::Type, TGV::Type, &Key>::new(&self.offset_key).insert_return_size(key, value)
    // }

    /// Get the `value` at `key` from the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given `key`.
    #[inline]
    pub fn get<'a>(&self, key: TGK::Type) -> Option<V>
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        RawMapping::<TGK::Type, V, &Key>::new(&self.offset_key).get(key)
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
        RawMapping::<TGK::Type, (), &Key>::new(&self.offset_key).size(key)
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
        RawMapping::<TGK::Type, (), &Key>::new(&self.offset_key).contains(key)
    }

    /// Clears the value at `key` from storage.
    pub fn remove<'a>(&self, key: TGK::Type)
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        RawMapping::<TGK::Type, V, &Key>::new(&self.offset_key).remove(key)
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
    use scale_info::{
        build::Fields,
        type_params,
        Path,
        Type,
        TypeInfo,
    };

    impl<K, V, TGK, TGV> TypeInfo for Mapping<K, V, TGK, TGV>
    where
        K: TypeInfo + 'static,
        V: TypeInfo + 'static,
        TGK: 'static,
        TGV: 'static,
    {
        type Identity = Self;

        fn type_info() -> Type {
            Type::builder()
                .path(Path::new("Mapping", module_path!()))
                .type_params(type_params![K, V])
                .composite(Fields::unnamed().field(|f| f.ty::<[(K, V)]>()))
        }
    }

    impl<K, V, TGK, TGV> StorageLayout for Mapping<K, V, TGK, TGV>
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
    fn insert_and_get_work() {
        let mut mapping: Mapping<u128, u128> = Mapping::default();

        mapping.insert(&1, &1);
        mapping.insert(&2, &2);

        assert_eq!(mapping.get(&1), Some(1));
        assert_eq!(mapping.get(&2), Some(2));
        assert_eq!(mapping.get(&3), None);
    }

    #[ink_lang::test]
    fn remove_and_contains_works() {
        let mut mapping: Mapping<u128, u128> = Mapping::default();

        mapping.insert(&1, &1);
        mapping.insert(&2, &2);

        assert_eq!(mapping.contains(&1), true);
        assert_eq!(mapping.contains(&2), true);

        mapping.remove(&1);

        assert_eq!(mapping.contains(&1), false);
        assert_eq!(mapping.contains(&2), true);
    }
}
