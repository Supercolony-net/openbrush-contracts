#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod my_flipper_guard {
    use flipper::traits::flipper::*;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, ReentrancyGuardStorage)]
    pub struct MyFlipper {
        #[ReentrancyGuardStorageField]
        guard: ReentrancyGuardData,
        value: bool,
    }

    impl FlipperStorage for MyFlipper {
        fn value(&self) -> &bool {
            &self.value
        }

        fn value_mut(&mut self) -> &mut bool {
            &mut self.value
        }
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }

    impl Flipper for MyFlipper {}
}
