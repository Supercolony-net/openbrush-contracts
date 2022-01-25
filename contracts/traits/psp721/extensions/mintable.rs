/// Extension of [`PSP721`] that exposes the mint function
pub use crate::traits::errors::PSP721Error;
pub use crate::traits::psp721::Id;
use brush::traits::AccountId;

#[brush::wrapper]
pub type PSP721MintableRef = dyn PSP721Mintable;

#[brush::trait_definition]
pub trait PSP721Mintable {
    /// Mints a new token with `id`.
    ///
    /// See [`PSP721::_mint`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error>;
}
