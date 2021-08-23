#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_pausable {
    use pausable::traits::*;

    #[ink(storage)]
    #[derive(Default, PausableStorage)]
    pub struct MyFlipper {
        #[PausableStorageField]
        pause: PausableData,
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[brush::modifiers(when_not_paused)]
        pub fn flip(&mut self) {
            self.flipped = !self.flipped;
        }

        #[ink(message)]
        pub fn pause(&mut self) {
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) {
            self._unpause()
        }
    }

    impl Pausable for MyFlipper {}
}
