#![cfg_attr(not(feature = "std"), no_std)]

/// It is stub implementation of contract with method `flip_on_me`.
/// We need this implementation to create wrapper for contract's account id.
/// With this wrapper we easy can call method of some contract.
/// Example:
/// ```
/// let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
/// flipper.flip_on_me();
/// ```
#[ink_lang::contract(compile_as_dependency = true)]
pub mod flip_on_me {
    #[ink(storage)]
    pub struct CallerOfFlip {}

    impl CallerOfFlip {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    impl CallerOfFlip {
        #[ink(message)]
        pub fn flip_on_me(&mut self) {
            unimplemented!()
        }
    }
}

#[brush::contract]
pub mod my_flipper_guard {
    use reentrancy_guard::traits::*;
    use brush::modifiers;
    use ink_env::call::FromAccountId;
    use crate::flip_on_me::CallerOfFlip;

    #[ink(storage)]
    #[derive(Default)]
    #[derive(ReentrancyGuardStorage)]
    pub struct MyFlipper {
        #[ReentrancyGuardStorageField]
        guard: ReentrancyGuardData,
        value: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }

        #[ink(message)]
        #[brush::modifiers(non_reentrant)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        #[modifiers(non_reentrant)]
        pub fn call_flip_on_me(&mut self, callee: AccountId) {
            // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
            // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
            // `call_flip_on_me` and `flip` is marked with `non_reentrant` modifier. It means,
            // that call of `flip` after `call_flip_on_me` must fails.
            let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
            flipper.flip_on_me();
        }
    }
}
