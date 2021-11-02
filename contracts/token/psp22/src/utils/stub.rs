#[ink_lang::contract(compile_as_dependency = true)]
pub mod token_timelock {
    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22TokenTimelock {}

    impl PSP22TokenTimelock {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22TokenTimelock")]
    impl PSP22TokenTimelock {
        #[ink(message)]
        pub fn token(&self) -> AccountId {
            unimplemented!()
        }

        #[ink(message)]
        pub fn beneficiary(&self) -> AccountId {
            unimplemented!()
        }

        #[ink(message)]
        pub fn release_time(&self) -> Timestamp {
            unimplemented!()
        }

        #[ink(message)]
        pub fn release(&mut self) {
            unimplemented!()
        }
    }
}
