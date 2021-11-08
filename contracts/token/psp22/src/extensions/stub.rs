#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp22metadata {
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
