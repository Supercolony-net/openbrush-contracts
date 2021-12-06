#![cfg_attr(not(feature = "std"), no_std)]

#[ink_lang::contract]
pub mod flip_on_me {
    use my_flipper_guard::my_flipper_guard::FlipperRef;
    use reentrancy_guard::traits::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct FlipOnMe {}

    impl FlipOnMe {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn flip_on_me(&mut self) -> Result<(), ReentrancyGuardError> {
            let caller = self.env().caller();
            self.flip_on_target(caller)
        }

        #[ink(message)]
        pub fn flip_on_target(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            // This method does a cross-contract call to caller contract and calls the `flip` method.
            FlipperRef::flip(&callee)
        }
    }
}
