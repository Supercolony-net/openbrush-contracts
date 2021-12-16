pub use crate::traits::errors::PSP22TokenTimelockError;
use brush::traits::{
    AccountId,
    Timestamp,
};

#[brush::wrapper]
pub type PSP22TokenTimelockRef = dyn PSP22TokenTimelock;

#[brush::trait_definition]
pub trait PSP22TokenTimelock {
    /// Returns the token address
    #[ink(message)]
    fn token(&self) -> AccountId;

    /// Returns the beneficiary of the tokens
    #[ink(message)]
    fn beneficiary(&self) -> AccountId;

    /// Returns the timestamp when the tokens are released
    #[ink(message)]
    fn release_time(&self) -> Timestamp;

    /// Transfers the tokens held by timelock to the beneficairy
    #[ink(message)]
    fn release(&mut self) -> Result<(), PSP22TokenTimelockError>;
}
