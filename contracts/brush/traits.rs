use ::ink_env::{
    Environment,
    DefaultEnvironment,
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