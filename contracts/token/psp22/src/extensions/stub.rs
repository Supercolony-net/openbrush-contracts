#[ink_lang::contract(compile_as_dependency = true)]
pub mod wrapper {
    use crate::traits::PSP22Error;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Wrapper {}

    impl PSP22Wrapper {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22Wrapper")]
    impl PSP22Wrapper {
        #[ink(message)]
        pub fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            unimplemented!()
        }
        #[ink(message)]
        pub fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp22flashmint {
    use crate::{
        extensions::stub::PSP3156FlashBorrower,
        traits::PSP22Error,
    };

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22FlashMint {}

    #[ink(namespace = "PSP22FlashMint")]
    impl PSP22FlashMint {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn max_flashloan(&mut self, token: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn flashloan(
            &mut self,
            receiver: &mut PSP3156FlashBorrower,
            token: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            unimplemented!()
        }
    }
}

pub use self::psp3156_flash_borrower::PSP3156FlashBorrower;

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp3156_flash_borrower {
    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP3156FlashBorrower {}

    #[ink(namespace = "PSP3156FlashBorrower")]
    impl PSP3156FlashBorrower {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_flash_loan(
            &mut self,
            initiator: AccountId,
            token: AccountId,
            amount: Balance,
            fee: Balance,
            data: Vec<u8>,
        ) -> [u8; 32] {
            unimplemented!()
        }
    }
}
