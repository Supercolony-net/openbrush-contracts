#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod flip_on_me {
    use flipper::traits::flip_on_me::*;
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

    impl FlipOnMe for FlipOnMeContract {}
}
