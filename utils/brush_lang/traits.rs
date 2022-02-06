use ::ink_env::{
    DefaultEnvironment,
    Environment,
};
use core::mem::ManuallyDrop;

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type Hash = <DefaultEnvironment as Environment>::Hash;
pub type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
pub type EnvAccess = ::ink_lang::EnvAccess<'static, DefaultEnvironment>;

#[crate::trait_definition]
pub trait InkStorage: Sized {
    fn env() -> EnvAccess {
        Default::default()
    }
}

impl<T> InkStorage for T {}

pub const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait AccountIdExt {
    fn is_zero(&self) -> bool;
}

impl AccountIdExt for AccountId {
    fn is_zero(&self) -> bool {
        self == &ZERO_ADDRESS.into()
    }
}

/// This trait is automatically implemented for storage.
#[crate::trait_definition]
pub trait Flush: ::ink_storage::traits::SpreadLayout + InkStorage {
    /// Method flushes the current state of `Self` into storage.
    /// ink! recursively calculate a key of each field.
    /// So if you want to flush the correct state of the contract,
    /// you have to this method on storage struct.
    fn flush(&self) {
        let root_key = ::ink_primitives::Key::from([0x00; 32]);
        ::ink_storage::traits::push_spread_root::<Self>(self, &root_key);
    }

    /// Method loads the current state of `Self` from storage.
    /// ink! recursively calculate a key of each field.
    /// So if you want to load the correct state of the contract,
    /// you have to this method on storage struct.
    fn load(&mut self) {
        let root_key = ::ink_primitives::Key::from([0x00; 32]);
        let mut state = ::ink_storage::traits::pull_spread_root::<Self>(&root_key);
        core::mem::swap(self, &mut state);
        let _ = ManuallyDrop::new(state);
    }
}

impl<T: ::ink_storage::traits::SpreadLayout + InkStorage> Flush for T {}
