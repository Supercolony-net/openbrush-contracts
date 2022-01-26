#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp721 {
    use brush::contracts::psp721::*;

    #[ink(storage)]
    #[derive(Default, PSP721Storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
        next_id: u8,
    }

    impl PSP721 for MyPSP721 {}

    impl MyPSP721 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP721Error> {
            self._mint([self.next_id; 32])?;
            self.next_id += 1;
            Ok(())
        }
    }
}
