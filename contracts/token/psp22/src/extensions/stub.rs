#[ink_lang::contract(compile_as_dependency = true)]
pub mod wrapper {
    use crate::stub::PSP22;

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
        pub fn init(&mut self, underlying: PSP22) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn deposit_for(&mut self, account: AccountId, amount: Balance) -> bool {
            unimplemented!()
        }

        #[ink(message)]
        pub fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> bool {
            unimplemented!()
        }

        pub fn _recover(&mut self, account: AccountId) -> Balance {
            unimplemented!()
        }
    }
}
