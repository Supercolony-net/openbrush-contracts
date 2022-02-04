/// Extension of [`PSP34`] that exposes the mint function
pub use crate::traits::errors::PSP34Error;
pub use crate::traits::psp34::Id;
use brush::traits::AccountId;

#[brush::wrapper]
pub type PSP34MintableRef = dyn PSP34Mintable;

#[brush::trait_definition]
pub trait PSP34Mintable {
    /// Mints a new token with `id`.
    ///
    /// See [`PSP34::_mint`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
