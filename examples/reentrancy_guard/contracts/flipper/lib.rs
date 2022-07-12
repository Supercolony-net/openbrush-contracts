#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_flipper_guard {
    use flipper::traits::flipper::*;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct MyFlipper {
        #[storage_field]
        guard: reentrancy_guard::Data,
        value: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }

    impl Flipper for MyFlipper {
        #[ink(message)]
        fn get_value(&self) -> bool {
            self.value
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        fn flip(&mut self) -> Result<(), ReentrancyGuardError> {
            self.value = !self.value;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(non_reentrant)]
        fn call_flip_on_me(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
            // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
            // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,
            // that call of `flip` after `call_flip_on_me` must fail.
            flipper::traits::flip_on_me::FlipOnMeRef::flip_on_me(&callee)
        }
    }
}
