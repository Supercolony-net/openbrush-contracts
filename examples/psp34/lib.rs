#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp34::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP34Storage)]
    pub struct MyPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
        next_id: u8,
    }

    impl PSP34 for MyPSP34 {}

    impl MyPSP34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), id)
        }
    }
}
