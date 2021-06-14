// It exports the stub implementation of all Erc1155 traits.
// ink! will generate a wrapper around all methods of each trait and it will allow creating wrapped
// struct around contracts address(::ink_env::call::FromAccountId::from_account_id).
pub use self::erc1155::{Erc1155};
pub use self::erc1155receiver::{Erc1155Receiver};

#[ink_lang::contract(compile_as_dependency = true)]
pub mod erc1155 {
    use crate::traits::{Id};
    use ink_prelude::{
        string::{String},
        vec::Vec,
    };

    #[ink(storage)]
    pub struct Erc1155 {}

    impl Erc1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc1155")]
    impl Erc1155 {
        #[ink(message)]
        pub fn balance_of(&self, _account: AccountId, _id: Id) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn balance_of_batch(&self, _owners: Vec<AccountId>, _ids: Vec<Id>) -> Vec<Balance> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn set_approval_for_all(&mut self, _operator: AccountId, _approved: bool) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
            unimplemented!()
        }

        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _amount: Balance,
            _data: Vec<u8>,
        ) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn safe_batch_transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _ids: Vec<Id>,
            _amounts: Vec<Balance>,
            _data: Vec<u8>,
        ) {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc1155MetadataURI")]
    impl Erc1155 {
        #[ink(message)]
        pub fn uri(&self, _id: Id) -> Option<String> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod erc1155receiver {
    use crate::traits::{Erc1155ReceiverError, Id};
    use ink_prelude::{
        vec::Vec,
    };

    #[ink(storage)]
    pub struct Erc1155Receiver {}

    impl Erc1155Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "IErc1155Receiver")]
    impl Erc1155Receiver {
        #[ink(message)]
        pub fn on_erc1155_received(&mut self, _operator: AccountId, _from: AccountId,
                                   _id: Id, _value: Balance, _data: Vec<u8>) -> Result<(), Erc1155ReceiverError> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_erc1155_batch_received(&mut self, _operator: AccountId, _from: AccountId,
                                         _ids: Vec<Id>, _values: Vec<Balance>, _data: Vec<u8>) -> Result<(), Erc1155ReceiverError> {
            unimplemented!()
        }
    }
}