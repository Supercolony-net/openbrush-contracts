#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_upgradeable {
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyPSP22 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        psp22: psp22::Data,
    }

    impl Ownable for MyPSP22 {}

    impl PSP22 for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            instance._init_with_owner(Self::env().caller());
            instance.initialize(total_supply).ok().unwrap();

            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(&mut self, total_supply: Balance) -> Result<(), OwnableError> {
            self._mint_to(self.owner(), total_supply).expect("Should mint");
            Ok(())
        }
    }
}
