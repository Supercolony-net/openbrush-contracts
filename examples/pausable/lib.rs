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
        #[brush::modifiers(when_paused)]
        pub fn flip(&mut self) {
            self.flipped = !self.flipped;
        }
    }

    impl Pausable for MyFlipper {}
}
