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

use ::ink::env::{
    DefaultEnvironment,
    Environment,
};
use core::mem::ManuallyDrop;
use ink::{
    prelude::vec::Vec,
    storage::traits::{
        Storable,
        StorageKey,
    },
};
pub use openbrush_lang_macro::Storage;

/// Aliases for types of the default environment
pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type Hash = <DefaultEnvironment as Environment>::Hash;
pub type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
pub type EnvAccess = ::ink::EnvAccess<'static, DefaultEnvironment>;
pub type String = Vec<u8>;

/// Each object has access to default environment via `Self::env()`.
/// It can be used for interaction with host functions of the blockchain.
pub trait DefaultEnv {
    #[inline(always)]
    fn env() -> EnvAccess {
        Default::default()
    }
}

impl<T> DefaultEnv for T {}

/// Implementation of the trait means that the type stores some `Data` inside.
/// It is stored in one exemplar, and reference can be retrieved from the object by `get` or
/// `get_mut` methods. The trait is helpful for generics implementations when you don't know
/// precisely the final type, but it is enough for you to know that it has some `Data` inside.
///
/// The trait is used as bound in OpenBrush to provide a generic implementation for contracts'
/// traits. The user of OpenBrush can "inherit" the default implementation by implementing the
/// `Storage<Data>` trait.
///
/// In most cases, the trait is implemented automatically by the derive macro.
/// The trait methods should not be used directly. Instead use the `data` method of
/// `StorageAsRef` or `StorageAsMut`.
pub trait Storage<Data>
where
    Data: OccupyStorage,
    Self: Flush + StorageAsRef + StorageAsMut + DefaultEnv,
{
    #[deprecated(since = "2.1.0", note = "please use `StorageAsRef::data` instead")]
    fn get(&self) -> &Data;

    #[deprecated(since = "2.1.0", note = "please use `StorageAsMut::data` instead")]
    fn get_mut(&mut self) -> &mut Data;
}

/// Trait describes that the storage `KEY` already is occupied by `WithData` type.
/// Implementation of that trait for each storage field prevents the user from occupying
/// the same storage cells.
pub trait OccupiedStorage<const KEY: u32> {
    type WithData: OccupyStorage;
}

/// Each upgradeable storage type should occupy its storage key. The trait helps to describe what
/// storage key is occupied by the type.
pub trait OccupyStorage {
    const KEY: u32;
}

/// Helper trait for `Storage` to provide user-friendly API to retrieve data as reference.
pub trait StorageAsRef {
    #[inline(always)]
    fn data<Data>(&self) -> &Data
    where
        Data: OccupyStorage,
        Self: Storage<Data>,
    {
        #[allow(deprecated)]
        <Self as Storage<Data>>::get(self)
    }
}

/// Helper trait for `Storage` to provide user-friendly API to retrieve data as mutable reference.
pub trait StorageAsMut: StorageAsRef {
    #[inline(always)]
    fn data<Data>(&mut self) -> &mut Data
    where
        Data: OccupyStorage,
        Self: Storage<Data>,
    {
        #[allow(deprecated)]
        <Self as Storage<Data>>::get_mut(self)
    }
}

impl<T> StorageAsRef for T {}
impl<T: StorageAsRef> StorageAsMut for T {}

pub const ZERO_ADDRESS: [u8; 32] = [255; 32];

/// The trait provides some useful methods for `AccountId` type.
pub trait AccountIdExt {
    fn is_zero(&self) -> bool;
}

impl AccountIdExt for AccountId {
    fn is_zero(&self) -> bool {
        self == &ZERO_ADDRESS.into()
    }
}

/// This trait is automatically implemented for storage structs.
pub trait Flush: Storable + Sized + StorageKey {
    /// Method flushes the current state of `Self` into storage.
    /// ink! recursively calculate a key of each field.
    /// So if you want to flush the correct state of the contract,
    /// you have to this method on storage struct.
    fn flush(&self) {
        ink::env::set_contract_storage(&Self::KEY, self);
    }

    /// Method loads the current state of `Self` from storage.
    /// ink! recursively calculate a key of each field.
    /// So if you want to load the correct state of the contract,
    /// you have to this method on storage struct.
    fn load(&mut self) {
        let mut state = ink::env::get_contract_storage(&Self::KEY)
            .unwrap_or_else(|error| panic!("Failed to load contract state: {:?}", error))
            .unwrap_or_else(|| panic!("Contract state is not initialized"));
        core::mem::swap(self, &mut state);
        let _ = ManuallyDrop::new(state);
    }
}

impl<T: Storable + Sized + StorageKey> Flush for T {}
