#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_upgradeable {
    use ink::storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
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
            ink::codegen::initialize_contract(|instance: &mut MyPSP22| {
                instance._init_with_owner(instance.env().caller());
                instance.initialize(total_supply).ok().unwrap()
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(&mut self, total_supply: Balance) -> Result<(), OwnableError> {
            self._mint_to(self.owner(), total_supply).expect("Should mint");
            Ok(())
        }
    }
}
