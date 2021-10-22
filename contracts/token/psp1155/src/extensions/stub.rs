#[ink_lang::contract(compile_as_dependency = true)]
pub mod metadata {
    use crate::traits::Id;
    use ink_prelude::string::String;

    #[ink(storage)]
    pub struct PSP1155Metadata {}

    impl PSP1155Metadata {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP1155Metadata")]
    impl PSP1155Metadata {
        #[ink(message)]
        pub fn uri(&self, _id: Id) -> Option<String> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod mintable {
    use crate::traits::Id;

    #[ink(storage)]
    pub struct PSP1155Mintable {}
    impl PSP1155Mintable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP1155Mintable")]
    impl PSP1155Mintable {
        #[ink(message)]
        pub fn mint(&mut self, id: Id, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId, id: Id, amount: Balance) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod burnable {
    use crate::traits::Id;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct PSP1155Burnable {}

    impl PSP1155Burnable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP1155Burnable")]
    impl PSP1155Burnable {
        #[ink(message)]
        pub fn burn(&mut self, id: Id, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn_from(&mut self, from: AccountId, id: Id, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn_batch(&mut self, ids_amounts: Vec<(Id, Balance)>) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn_batch_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) {
            unimplemented!()
        }
    }
}
