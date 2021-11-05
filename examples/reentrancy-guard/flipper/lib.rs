#![cfg_attr(not(feature = "std"), no_std)]

/// This is a stub implementation of contract with method `flip_on_me`.
/// We need this implementation to create wrapper for contract's account id.
/// With this wrapper, we can easily call methods of some contract.
/// Example:
/// ```
/// let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
/// flipper.flip_on_me();
/// ```
#[ink_lang::contract(compile_as_dependency = true)]
pub mod flip_on_me {
    use reentrancy_guard::traits::*;
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
        pub fn flip_on_me(&mut self) -> Result<(), ReentrancyGuardError> {
            unimplemented!()
        }
    }
}

#[brush::contract]
pub mod my_flipper_guard {
    use brush::modifiers;
    use reentrancy_guard::traits::*;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_env::call::FromAccountId;

    #[cfg(not(feature = "ink-as-dependency"))]
    use crate::flip_on_me::CallerOfFlip;

    #[ink(storage)]
    #[derive(Default, ReentrancyGuardStorage)]
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
        pub fn flip(&mut self) -> Result<(), ReentrancyGuardError> {
            self.value = !self.value;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(non_reentrant)]
        pub fn call_flip_on_me(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError> {
            // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
            // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
            // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,
            // that call of `flip` after `call_flip_on_me` must fail.
            let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
            flipper.flip_on_me()
        }
    }
}
