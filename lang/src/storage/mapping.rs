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

use crate::storage::RefGuard;
use ink::{
    primitives::Key,
    storage::traits::{
        AutoKey,
        Packed,
        Storable,
        StorableHint,
        StorageKey,
    },
};
use scale::{
    Error,
    Input,
    Output,
};

/// It is a more restricted version of the `Mapping` from ink!. That mapping can be used to unify
/// the API calls to the `Mapping` to avoid monomorphization to reduce the size of contracts.
/// It verifies that all calls are done with the same type during compilation.
pub struct Mapping<K, V, TGK = RefGuard<K>, TGV = ValueGuard<V>, KeyType: StorageKey = AutoKey> {
    #[allow(clippy::type_complexity)]
    _marker: PhantomData<fn() -> (K, V, KeyType, TGK, TGV)>,
}

/// We implement this manually because the derived implementation adds trait bounds.
impl<K, V, TGK, TGV, KeyType> Default for Mapping<K, V, TGK, TGV, KeyType>
where
    V: Packed,
    KeyType: StorageKey,
{
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<K, V, TGK, TGV, KeyType> core::fmt::Debug for Mapping<K, V, TGK, TGV, KeyType>
where
    V: Packed,
    KeyType: StorageKey,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mapping").field("key", &KeyType::KEY).finish()
    }
}

impl<K, V, TGK, TGV, KeyType> Mapping<K, V, TGK, TGV, KeyType>
where
    KeyType: StorageKey,
{
    /// Creates a new empty `Mapping`.
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<K, V, TGK, TGV, KeyType> Mapping<K, V, TGK, TGV, KeyType>
where
    K: Packed,
    V: Packed,
    KeyType: StorageKey,
{
    /// Insert the given `value` to the contract storage.
    #[inline]
    pub fn insert<'a, 'b>(&mut self, key: TGK::Type, value: &TGV::Type)
    where
        TGK: TypeGuard<'a>,
        TGV: TypeGuard<'b>,
        TGK::Type: scale::Encode,
        TGV::Type: Packed,
    {
        RawMapping::<TGK::Type, TGV::Type, &Key>::new(&KeyType::KEY).insert(key, value)
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
        RawMapping::<TGK::Type, V, &Key>::new(&KeyType::KEY).get(key)
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
        RawMapping::<TGK::Type, (), &Key>::new(&KeyType::KEY).size(key)
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
        RawMapping::<TGK::Type, (), &Key>::new(&KeyType::KEY).contains(key)
    }

    /// Clears the value at `key` from storage.
    pub fn remove<'a>(&self, key: TGK::Type)
    where
        TGK: TypeGuard<'a>,
        TGK::Type: scale::Encode,
    {
        RawMapping::<TGK::Type, V, &Key>::new(&KeyType::KEY).remove(key)
    }
}

impl<K, V, TGK, TGV, KeyType> Storable for Mapping<K, V, TGK, TGV, KeyType>
where
    V: Packed,
    KeyType: StorageKey,
{
    #[inline]
    fn encode<T: Output + ?Sized>(&self, _dest: &mut T) {}

    #[inline]
    fn decode<I: Input>(_input: &mut I) -> Result<Self, Error> {
        Ok(Default::default())
    }
}

impl<K, V, Key, TGK, TGV, InnerKey> StorableHint<Key> for Mapping<K, V, TGK, TGV, InnerKey>
where
    V: Packed,
    Key: StorageKey,
    InnerKey: StorageKey,
{
    type Type = Mapping<K, V, TGK, TGV, Key>;
    type PreferredKey = InnerKey;
}

impl<K, V, TGK, TGV, KeyType> StorageKey for Mapping<K, V, TGK, TGV, KeyType>
where
    V: Packed,
    KeyType: StorageKey,
{
    const KEY: Key = KeyType::KEY;
}

#[cfg(feature = "std")]
const _: () = {
    use ink::{
        metadata::layout::{
            Layout,
            LayoutKey,
            RootLayout,
        },
        storage::traits::StorageLayout,
    };
    use scale_info::{
        build::Fields,
        type_params,
        Path,
        Type,
        TypeInfo,
    };

    impl<K, V, TGK, TGV, KeyType> TypeInfo for Mapping<K, V, TGK, TGV, KeyType>
    where
        K: TypeInfo + 'static,
        V: TypeInfo + 'static,
        TGK: 'static,
        TGV: 'static,
        KeyType: StorageKey + 'static,
    {
        type Identity = Self;

        fn type_info() -> Type {
            Type::builder()
                .path(Path::new("Mapping", module_path!()))
                .type_params(type_params![K, V])
                .composite(Fields::unnamed().field(|f| f.ty::<[(K, V)]>()))
        }
    }

    impl<K, V, TGK, TGV, KeyType> StorageLayout for Mapping<K, V, TGK, TGV, KeyType>
    where
        K: scale_info::TypeInfo + 'static,
        V: Packed + StorageLayout + scale_info::TypeInfo + 'static,
        KeyType: StorageKey + 'static,
        TGK: 'static,
        TGV: 'static,
    {
        fn layout(_: &Key) -> Layout {
            Layout::Root(RootLayout::new(
                LayoutKey::from(&KeyType::KEY),
                <V as StorageLayout>::layout(&KeyType::KEY),
            ))
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;
    #[ink::test]
    fn insert_and_get_work() {
        let mut mapping: Mapping<u128, u128> = Mapping::default();

        mapping.insert(&1, &1);
        mapping.insert(&2, &2);

        assert_eq!(mapping.get(&1), Some(1));
        assert_eq!(mapping.get(&2), Some(2));
        assert_eq!(mapping.get(&3), None);
    }

    #[ink::test]
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
