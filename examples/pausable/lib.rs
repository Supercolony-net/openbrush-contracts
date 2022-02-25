#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_pausable {
    use brush::contracts::pausable::*;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PausableStorage)]
    pub struct MyFlipper {
        #[PausableStorageField]
        pause: PausableData,
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        #[brush::modifiers(when_not_paused)]
        pub fn flip(&mut self) -> Result<(), PausableError> {
            self.flipped = !self.flipped;
            Ok(())
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
            self._unpause()
        }
    }

    impl Pausable for MyFlipper {}
}
