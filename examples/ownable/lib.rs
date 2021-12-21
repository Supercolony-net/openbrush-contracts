#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_ownable {
    use brush::{
        contracts::ownable::*,
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, OwnableStorage)]
    pub struct MyOwnable {
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    impl Ownable for MyOwnable {}

    impl MyOwnable {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn owner_function(&mut self) -> Result<(), OwnableError> {
            todo!()
        }
    }
}
