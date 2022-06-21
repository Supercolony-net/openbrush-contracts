pub use openbrush::contracts::reentrancy_guard::*;
use openbrush::{
    modifiers,
    traits::AccountId,
};
use openbrush::traits::Storage;

#[openbrush::wrapper]
pub type FlipperRef = dyn Flipper;

#[openbrush::trait_definition]
pub trait Flipper: Storage<bool> + Storage<Data> {
    #[ink(message)]
    fn get_value(&self) -> bool {
        *<Self as Storage<bool>>::get(self)
    }

    #[ink(message)]
    #[openbrush::modifiers(non_reentrant)]
    fn flip(&mut self) -> Result<(), ReentrancyGuardError> {
        *<Self as Storage<bool>>::get_mut(self) = !*<Self as Storage<bool>>::get(self);
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
