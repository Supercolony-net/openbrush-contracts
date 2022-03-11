pub use brush::contracts::reentrancy_guard::*;
use brush::traits::{
    AccountId,
    InkStorage,
};

#[brush::wrapper]
pub type FlipOnMeRef = dyn FlipOnMe;

#[brush::trait_definition]
pub trait FlipOnMe: InkStorage {
    #[ink(message)]
    fn flip_on_me(&mut self) -> Result<(), ReentrancyGuardError> {
        let caller = Self::env().caller();
        self.flip_on_target(caller)
    }

    #[ink(message)]
    fn flip_on_target(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
        // This method does a cross-contract call to caller contract and calls the `flip` method.
        crate::traits::flipper::FlipperRef::flip(&callee)
    }
}
