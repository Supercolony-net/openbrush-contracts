/// Extension of [`PSP721`] that allows token holders to destroy their tokens
pub use crate::traits::errors::PSP721Error;
pub use crate::traits::psp721::Id;
use brush::traits::AccountId;

#[brush::wrapper]
pub type PSP721BurnableRef = dyn PSP721Burnable;

#[brush::trait_definition]
pub trait PSP721Burnable {
    /// Destroys token with id equal to `id` from `account`
    ///
    /// Caller must be approved to transfer tokens from `account`
    /// or to transfer token with `id`
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error>;
}
