pub use openbrush::contracts::reentrancy_guard::*;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type FlipOnMeRef = dyn FlipOnMe;

#[openbrush::trait_definition]
pub trait FlipOnMe {
    #[ink(message)]
    fn flip_on_me(&mut self) -> Result<(), ReentrancyGuardError>;

    #[ink(message)]
    fn flip_on_target(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError>;
}
