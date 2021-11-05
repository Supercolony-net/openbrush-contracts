#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod revert_on_error_poc {
    use ink_prelude::string::String;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MyError {
        SomeError,
        AnotherError,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Storage {
        flip: bool,
    }

    impl Storage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.flip
        }
        #[ink(message)]
        pub fn flip(&mut self) -> core::result::Result<(), MyError> {
            self.flip = !self.flip;
            Ok(())
        }
        #[ink(message)]
        pub fn flip_with_error(&mut self) -> core::result::Result<(), MyError> {
            self.flip = !self.flip;
            core::result::Result::Err(MyError::SomeError)
        }

        #[ink(message)]
        pub fn flip_with_args(&mut self, number: u128) -> core::result::Result<(), MyError> {
            self.flip = !self.flip;
            Ok(())
        }
    }
}
