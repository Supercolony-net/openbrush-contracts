#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp34 {
    use brush::contracts::psp34::*;

    #[ink(storage)]
    #[derive(Default, PSP34Storage)]
    pub struct MyPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
        next_id: u8,
    }

    impl PSP34 for MyPSP34 {}

    impl MyPSP34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint([self.next_id; 32])?;
            self.next_id += 1;
            Ok(())
        }
    }
}
