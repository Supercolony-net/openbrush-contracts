#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod flip_on_me {
    use flipper::traits::flip_on_me::*;
    use ink_lang::codegen::Env;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct FlipOnMeContract {}

    impl FlipOnMeContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }

    impl FlipOnMe for FlipOnMeContract {
        #[ink(message)]
        fn flip_on_me(&mut self) -> Result<(), ReentrancyGuardError> {
            let caller = self.env().caller();
            self.flip_on_target(caller)
        }

        #[ink(message)]
        fn flip_on_target(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            // This method does a cross-contract call to caller contract and calls the `flip` method.
            flipper::traits::flipper::FlipperRef::flip(&callee)
        }
    }
}
