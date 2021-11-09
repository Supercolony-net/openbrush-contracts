#[ink_lang::contract(compile_as_dependency = true)]
pub mod metadata {
    use ink_prelude::string::String;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP721Metadata {}

    impl PSP721Metadata {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP721Metadata")]
    impl PSP721Metadata {
        #[ink(message)]
        pub fn name(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn symbol(&self) -> Option<String> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod mintable {
    use crate::traits::{
        Id,
        PSP721Error,
    };

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP721Mintable {}

    impl PSP721Mintable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP721Mintable")]
    impl PSP721Mintable {
        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP721Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn mint_to(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod burnable {
    use crate::traits::{
        Id,
        PSP721Error,
    };

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP721Burnable {}

    impl PSP721Burnable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP721Burnable")]
    impl PSP721Burnable {
        #[ink(message)]
        pub fn burn(&mut self, id: Id) -> Result<(), PSP721Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn_from(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
            unimplemented!()
        }
    }
}
