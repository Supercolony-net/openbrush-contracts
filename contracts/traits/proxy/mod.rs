pub use crate::traits::{errors::OwnableError, ownable::*};
use brush::traits::Hash;

#[brush::wrapper]
pub type ProxyRef = dyn Proxy + Ownable;

#[brush::trait_definition]
pub trait Proxy: Ownable {
    #[ink(message)]
    fn get_delegate_code(&self) -> Hash;

    #[ink(message)]
    fn change_delegate_code(&mut self, new_code_hash: Hash) -> Result<(), OwnableError>;
}
