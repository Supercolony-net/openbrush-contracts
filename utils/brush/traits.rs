use ::ink_env::{
    DefaultEnvironment,
    Environment,
};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type EnvAccess = ::ink_lang::EnvAccess<'static, DefaultEnvironment>;

pub trait InkStorage {
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

pub trait Flush {
    /// Method flushes the current state of `Self` into storage.
    /// ink! recursively calculate the key of each field.
    /// So if you want to flush the correct state of the contract,
    /// you must call this method on storage struct.
    ///
    /// ** Note ** `#[brush::contract]` macro provides implementation of `Flush` trait
    /// by default for storage.
    fn flush(&self);
}
