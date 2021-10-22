#[ink_lang::contract(compile_as_dependency = true)]
mod psp22_wrapper {

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
        fn init(&mut self, underlying: u8) {
            unimplemented!()
        }
        #[ink(message)]
        fn deposit_for(&mut self, account: AccountId, amount: Balance) -> bool {
            unimplemented!()
        }
        #[ink(message)]
        fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> bool {
            unimplemented!()
        }
        fn _recover(&mut self, account: AccountId) -> Balance {
            unimplemented!()
        }
    }
}
