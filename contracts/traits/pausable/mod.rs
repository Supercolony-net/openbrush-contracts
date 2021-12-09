pub use crate::traits::errors::PausableError;

#[brush::wrapper]
pub type PausableRef = dyn Pausable;

/// Contract trait, which allows children to implement an emergency stop
/// mechanism that an authorized account can trigger.
#[brush::trait_definition]
pub trait Pausable {
    /// Returns true if the contract is paused, and false otherwise.
    #[ink(message)]
    fn paused(&self) -> bool;
}
