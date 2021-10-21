#[ink_lang::contract(compile_as_dependency = true)]
mod psp1155_metadata {
    use ink_prelude::string::String;

    #[ink(namespace = "PSP1155Metadata")]
    impl PSP1155 {
        #[ink(message)]
        fn uri(&self, _id: Id) -> Option<String> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
mod psp1155_mintable {
    use brush::traits::{
        AccountId,
        Balance,
    };

    #[ink(namespace = "PSP1155Mintable")]
    impl PSP1155 {
        #[ink(message)]
        fn mint(&mut self, id: Id, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        fn mint_to(&mut self, to: AccountId, id: Id, amount: Balance) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
mod psp1155_burnable {
    use brush::traits::{
        AccountId,
        Balance,
    };
    use ink_prelude::vec::Vec;

    #[ink(namespace = "PSP1155Burnable")]
    impl PSP1155 {
        #[ink(message)]
        fn burn(&mut self, id: Id, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        fn burn_from(&mut self, from: AccountId, id: Id, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        fn burn_batch(&mut self, ids_amounts: Vec<(Id, Balance)>) {
            unimplemented!()
        }

        #[ink(message)]
        fn burn_batch_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) {
            unimplemented!()
        }
    }
}
