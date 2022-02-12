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
    use brush::contracts::reentrancy_guard::*;
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
    use brush::{
        contracts::reentrancy_guard::*,
        modifiers,
    };

    use crate::flip_on_me::CallerOfFlipRef;
    use ink_env::call::FromAccountId;

    pub trait FlipperStorage {
        fn value(&self) -> &bool;
        fn value_mut(&mut self) -> &mut bool;
    }

    // TODO: Refactor example to have a separate trait defined in ../ to avoid `ink-as-dependency`
    #[brush::wrapper]
    pub type FlipperRef = dyn Flipper;

    #[brush::trait_definition]
    pub trait Flipper: FlipperStorage + ReentrancyGuardStorage {
        #[ink(message)]
        fn get_value(&self) -> bool {
            self.value().clone()
        }

        #[ink(message)]
        #[brush::modifiers(non_reentrant)]
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
            let mut flipper: CallerOfFlipRef = FromAccountId::from_account_id(callee);
            flipper.flip_on_me()
        }
    }

    #[ink(storage)]
    #[derive(Default, ReentrancyGuardStorage)]
    pub struct MyFlipper {
        #[ReentrancyGuardStorageField]
        guard: ReentrancyGuardData,
        value: bool,
    }

    impl FlipperStorage for MyFlipper {
        fn value(&self) -> &bool {
            &self.value
        }

        fn value_mut(&mut self) -> &mut bool {
            &mut self.value
        }
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Flipper for MyFlipper {}
}
