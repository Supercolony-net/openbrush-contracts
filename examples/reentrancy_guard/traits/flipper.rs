pub use openbrush::contracts::reentrancy_guard::*;
use openbrush::{
    modifiers,
    traits::AccountId,
};

pub trait FlipperStorage {
    fn value(&self) -> &bool;
    fn value_mut(&mut self) -> &mut bool;
}

#[openbrush::wrapper]
pub type FlipperRef = dyn Flipper;

#[openbrush::trait_definition]
pub trait Flipper: FlipperStorage + ReentrancyGuardStorage<Data = ReentrancyGuardData> {
    #[ink(message)]
    fn get_value(&self) -> bool {
        self.value().clone()
    }

    #[ink(message)]
    #[openbrush::modifiers(non_reentrant)]
    fn flip(&mut self) -> Result<(), ReentrancyGuardError> {
        *self.value_mut() = !self.value().clone();
        Ok(())
    }

    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn call_flip_on_me(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
        // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
        // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
        // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,
        // that call of `flip` after `call_flip_on_me` must fail.
        crate::traits::flip_on_me::FlipOnMeRef::flip_on_me(&callee)
    }
}
