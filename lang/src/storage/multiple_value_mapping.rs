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

use ink_storage::{
    traits::{
        PackedLayout,
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};
use scale::Ref;

/// A mapping of key-value pairs directly into contract storage.
///
/// # Important
///
/// If you use this data structure you must use the function
/// [`ink_lang::utils::initialize_contract`](https://paritytech.github.io/ink/ink_lang/utils/fn.initialize_contract.html)
/// in your contract's constructors!
///
/// Note that in order to use this function your contract's storage struct must implement the
/// [`SpreadAllocate`](crate::traits::SpreadAllocate) trait.
///
/// This is an example of how you can do this:
/// ```rust
/// # use ink_lang as ink;
/// # use ink_env::{
/// #     Environment,
/// #     DefaultEnvironment,
/// # };
/// # type AccountId = <DefaultEnvironment as Environment>::AccountId;
///
/// # #[ink::contract]
/// # mod my_module {
/// use ink_storage::{traits::SpreadAllocate, Mapping};
///
/// #[ink(storage)]
/// #[derive(SpreadAllocate)]
/// pub struct MyContract {
///     balances: Mapping<AccountId, Balance>,
/// }
///
/// impl MyContract {
///     #[ink(constructor)]
///     pub fn new() -> Self {
///         ink_lang::utils::initialize_contract(Self::new_init)
///     }
///
///     /// Default initializes the contract.
///     fn new_init(&mut self) {
///         let caller = Self::env().caller();
///         let value: Balance = Default::default();
///         self.balances.insert(&caller, &value);
///     }
/// #   #[ink(message)]
/// #   pub fn my_message(&self) { }
/// }
/// # }
/// ```
///
/// More usage examples can be found [in the ink! examples](https://github.com/paritytech/ink/tree/master/examples).
#[derive(SpreadLayout, SpreadAllocate)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct MultipleValueMapping<K, V> {
    /// Contains count of values by key.
    key_count: Mapping<K, u128>,
    /// Mapping from key's value to local index.
    value_to_index: Mapping<(K, V), u128>,
    /// Mapping from local key's index to value.
    index_to_value: Mapping<(K, u128), V>,
}

impl<K, V> Default for MultipleValueMapping<K, V> {
    fn default() -> Self {
        Self {
            key_count: Default::default(),
            value_to_index: Default::default(),
            index_to_value: Default::default(),
        }
    }
}

impl<K, V> core::fmt::Debug for MultipleValueMapping<K, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultipleValueMapping")
            .field("key_count", &self.key_count)
            .field("value_to_index", &self.value_to_index)
            .field("index_to_value", &self.index_to_value)
            .finish()
    }
}

impl<K, V> MultipleValueMapping<K, V>
where
    K: PackedLayout,
    V: PackedLayout,
    V: scale::EncodeLike<V>,
{
    /// Insert the given `value` to the contract storage at `key`.
    pub fn insert<Q, R>(&mut self, key: &Q, value: &R)
    where
        Q: scale::EncodeLike<K>,
        R: scale::EncodeLike<V> + PackedLayout,
    {
        self.insert_return_size::<Q, R>(key, value);
    }

    /// Insert the given `value` to the contract storage at `key`.
    ///
    /// Returns the size of the pre-existing value at the specified key if any.
    pub fn insert_return_size<Q, R>(&mut self, key: &Q, value: &R) -> Option<u32>
    where
        Q: scale::EncodeLike<K>,
        R: scale::EncodeLike<V> + PackedLayout,
    {
        let count = self.count::<Q>(key);
        self.value_to_index.insert(
            (
                <Ref<'_, Q, K> as From<_>>::from(key),
                <Ref<'_, R, V> as From<_>>::from(value),
            ),
            &count,
        );
        self.index_to_value
            .insert_return_size((<Ref<'_, Q, K> as From<_>>::from(key), &count), value)
    }

    /// Returns the count of values stored under the `key`.
    #[inline]
    pub fn count<Q>(&self, key: &Q) -> u128
    where
        Q: scale::EncodeLike<K>,
    {
        self.key_count
            .get(<Ref<'_, Q, K> as From<_>>::from(key))
            .unwrap_or_default()
    }

    /// Get the `value` at (`key`, `index`) from the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given (`key`, `index`).
    #[inline]
    pub fn get_value<Q>(&self, key: &Q, index: &u128) -> Option<V>
    where
        Q: scale::EncodeLike<K>,
    {
        self.index_to_value.get((<Ref<'_, Q, K> as From<_>>::from(key), index))
    }

    /// Get the `index` of (`key`, `value`) from the contract storage.
    ///
    /// Returns `None` if no `value` exists for the given `key`.
    #[inline]
    pub fn get_index<Q, R>(&self, key: &Q, value: &R) -> Option<u128>
    where
        Q: scale::EncodeLike<K>,
        R: scale::EncodeLike<V>,
    {
        self.value_to_index.get((
            <Ref<'_, Q, K> as From<_>>::from(key),
            <Ref<'_, R, V> as From<_>>::from(value),
        ))
    }

    /// Get the size of a value stored at (`key`, `index`) in the contract storage.
    ///
    /// Returns `None` if no `value` exists at the given (`key`, `index`).
    #[inline]
    pub fn size<Q>(&self, key: &Q, index: &u128) -> Option<u32>
    where
        Q: scale::EncodeLike<K>,
    {
        self.index_to_value.size((<Ref<'_, Q, K> as From<_>>::from(key), index))
    }

    /// Checks if any value is stored at the given `key` in the contract storage.
    #[inline]
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: scale::EncodeLike<K>,
    {
        self.count::<Q>(key) > 0
    }

    /// Checks if the `value` is stored at the given `key` in the contract storage.
    #[inline]
    pub fn contains_value<Q, R>(&self, key: &Q, value: &R) -> bool
    where
        Q: scale::EncodeLike<K>,
        R: scale::EncodeLike<V>,
    {
        self.value_to_index.contains((
            <Ref<'_, Q, K> as From<_>>::from(key),
            <Ref<'_, R, V> as From<_>>::from(value),
        ))
    }

    /// Checks if any value is stored at the given (`key`, `index`) in the contract storage.
    #[inline]
    pub fn contains_index<Q>(&self, key: &Q, index: &u128) -> bool
    where
        Q: scale::EncodeLike<K>,
    {
        self.index_to_value
            .contains((<Ref<'_, Q, K> as From<_>>::from(key), index))
    }

    /// Clears the `value` at `key` from storage.
    pub fn remove_value<Q, R>(&mut self, key: &Q, value: &R)
    where
        Q: scale::EncodeLike<K>,
        R: scale::EncodeLike<V>,
        V: scale::EncodeLike<V>,
    {
        let op_index = self.get_index::<Q, R>(key, value);

        let index;
        if let Some(op_index) = op_index {
            index = op_index;
        } else {
            return
        }
        let index = &index;
        self.swap_and_remove::<Q, R>(key, value, index);
    }

    /// Clears the value at (`key`, `index`) from storage.
    pub fn remove_index<Q>(&mut self, key: &Q, index: &u128)
    where
        Q: scale::EncodeLike<K>,
    {
        let op_value = self.get_value::<Q>(key, index);

        let value;
        if let Some(op_value) = op_value {
            value = op_value;
        } else {
            return
        }
        let value = &value;
        self.swap_and_remove::<Q, V>(key, value, index);
    }

    fn swap_and_remove<Q, R>(&mut self, key: &Q, value: &R, index: &u128)
    where
        Q: scale::EncodeLike<K>,
        R: scale::EncodeLike<V>,
    {
        let last_index = &self.count(key);

        if last_index != index {
            let last_value = &self
                .index_to_value
                .get((<Ref<'_, Q, K> as From<_>>::from(key), &last_index))
                .expect("The value under the last index should exist");
            self.index_to_value
                .insert((<Ref<'_, Q, K> as From<_>>::from(key), &index), last_value);
            self.value_to_index.insert(
                (
                    <Ref<'_, Q, K> as From<_>>::from(key),
                    <Ref<'_, V, V> as From<_>>::from(last_value),
                ),
                index,
            );
        }

        self.index_to_value
            .remove((<Ref<'_, Q, K> as From<_>>::from(key), &last_index));
        self.value_to_index.remove((
            <Ref<'_, Q, K> as From<_>>::from(key),
            <Ref<'_, R, V> as From<_>>::from(value),
        ));
    }
}

#[cfg(feature = "std")]
const _: () = {
    use ink_metadata::layout::{
        FieldLayout,
        Layout,
        StructLayout,
    };
    use ink_primitives::KeyPtr;
    use ink_storage::traits::StorageLayout;

    impl<K, V> StorageLayout for MultipleValueMapping<K, V>
    where
        K: scale_info::TypeInfo + 'static,
        V: scale_info::TypeInfo + 'static,
    {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            Layout::Struct(StructLayout::new([
                FieldLayout::new(
                    Some("key_count"),
                    <Mapping<K, u128> as ::ink_storage::traits::StorageLayout>::layout(key_ptr),
                ),
                FieldLayout::new(
                    Some("value_to_index"),
                    <Mapping<(K, V), u128> as ::ink_storage::traits::StorageLayout>::layout(key_ptr),
                ),
                FieldLayout::new(
                    Some("index_to_value"),
                    <Mapping<(K, u128), V> as ::ink_storage::traits::StorageLayout>::layout(key_ptr),
                ),
            ]))
        }
    }
};

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn insert_and_get_work() {
//         ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
//             let mut mapping: Mapping<u8, _> = Mapping::new([0u8; 32].into());
//             mapping.insert(&1, &2);
//             assert_eq!(mapping.get(&1), Some(2));
//
//             Ok(())
//         })
//         .unwrap()
//     }
//
//     #[test]
//     fn gets_default_if_no_key_set() {
//         ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
//             let mapping: Mapping<u8, u8> = Mapping::new([0u8; 32].into());
//             assert_eq!(mapping.get(&1), None);
//
//             Ok(())
//         })
//         .unwrap()
//     }
//
//     #[test]
//     fn can_clear_entries() {
//         ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
//             // We use `Pack` here since it `REQUIRES_DEEP_CLEAN_UP`
//             use crate::pack::Pack;
//
//             // Given
//             let mut mapping: Mapping<u8, u8> = Mapping::new([0u8; 32].into());
//             let mut deep_mapping: Mapping<u8, Pack<u8>> = Mapping::new([1u8; 32].into());
//
//             mapping.insert(&1, &2);
//             assert_eq!(mapping.get(&1), Some(2));
//
//             deep_mapping.insert(&1u8, &Pack::new(Pack::new(2u8)));
//             assert_eq!(deep_mapping.get(&1), Some(Pack::new(2u8)));
//
//             // When
//             mapping.remove(&1);
//             deep_mapping.remove(&1);
//
//             // Then
//             assert_eq!(mapping.get(&1), None);
//             assert_eq!(deep_mapping.get(&1), None);
//
//             Ok(())
//         })
//         .unwrap()
//     }
//
//     #[test]
//     fn can_clear_unexistent_entries() {
//         ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
//             // We use `Pack` here since it `REQUIRES_DEEP_CLEAN_UP`
//             use crate::pack::Pack;
//
//             // Given
//             let mapping: Mapping<u8, u8> = Mapping::new([0u8; 32].into());
//             let deep_mapping: Mapping<u8, Pack<u8>> = Mapping::new([1u8; 32].into());
//
//             // When
//             mapping.remove(&1);
//             deep_mapping.remove(&1);
//
//             // Then
//             assert_eq!(mapping.get(&1), None);
//             assert_eq!(deep_mapping.get(&1), None);
//
//             Ok(())
//         })
//         .unwrap()
//     }
// }
