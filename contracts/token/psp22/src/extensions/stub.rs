#[ink_lang::contract(compile_as_dependency = true)]
pub mod metadata {
    use ink_prelude::string::String;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Metadata {}

    impl PSP22Metadata {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22Metadata")]
    impl PSP22Metadata {
        #[ink(message)]
        pub fn token_name(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_symbol(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_decimals(&self) -> u8 {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod mintable {
    use common::errors::PSP22Error;
    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Mintable {}

    impl PSP22Mintable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22Mintable")]
    impl PSP22Mintable {
        #[ink(message)]
        pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod burnable {
    use common::errors::PSP22Error;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Burnable {}

    impl PSP22Burnable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22Burnable")]
    impl PSP22Burnable {
        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            unimplemented!()
        }
    }
}
