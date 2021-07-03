#![cfg_attr(not(feature = "std"), no_std)]

/// It is stub implementation of contract with method `flip`.
/// We need this implementation to create wrapper around account id of contract.
/// With this wrapper we easy can call method of some contract.
/// Example:
/// ```
/// let mut flipper: Flipper = FromAccountId::from_account_id(caller);
/// flipper.flip();
/// ```
#[ink_lang::contract(compile_as_dependency = true)]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {}

    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    impl Flipper {
        #[ink(message)]
        pub fn flip(&mut self) {
            unimplemented!()
        }
    }
}

#[brush::contract]
pub mod flip_on_me {
    use ink_env::call::FromAccountId;
    use crate::flipper::Flipper;

    #[ink(storage)]
    #[derive(Default)]
    pub struct FlipOnMe {}

    impl FlipOnMe {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn flip_on_me(&mut self) {
            let caller = self.env().caller();
            // This method will do a cross-contract call to caller account. It will try to call `flip`
            let mut flipper: Flipper = FromAccountId::from_account_id(caller);
            flipper.flip();
        }
    }
}
