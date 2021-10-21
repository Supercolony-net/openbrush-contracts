#[ink_lang::contract(compile_as_dependency = true)]
mod psp721_metadata {
    use ink_prelude::string::String;

    #[ink(namespace = "PSP721Metadata")]
    impl PSP721 {
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
mod psp721_mintable {
    use brush::traits::AccountId;

    #[ink(namespace = "PSP721Mintable")]
    impl PSP721 {
        #[ink(message)]
        fn mint(&mut self, id: Id) {
            unimplemented!()
        }

        #[ink(message)]
        fn mint_to(&mut self, account: AccountId, id: Id) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
mod psp721_burnable {
    use brush::traits::AccountId;

    #[ink(namespace = "PSP721Burnable")]
    impl PSP721 {
        #[ink(message)]
        fn burn(&mut self, id: Id) {
            unimplemented!()
        }

        #[ink(message)]
        fn burn_from(&mut self, account: AccountId, id: Id) {
            unimplemented!()
        }
    }
}
