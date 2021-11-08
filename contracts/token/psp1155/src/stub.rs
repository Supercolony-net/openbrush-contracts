// It exports the stub implementation of all PSP1155 traits.
// ink! will generate a wrapper around all methods of each trait and it will allow creating wrapped
// struct around contracts address(::ink_env::call::FromAccountId::from_account_id).
pub use self::{
    psp1155::PSP1155,
    psp1155receiver::PSP1155Receiver,
};

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp1155 {
    use crate::traits::Id;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct PSP1155 {}

    impl PSP1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP1155")]
    impl PSP1155 {
        #[ink(message)]
        pub fn balance_of(&self, _account: AccountId, _id: Id) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn balance_of_batch(&self, _owners_ids: Vec<(AccountId, Id)>) -> Vec<Balance> {
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
            _ids_to_amounts: Vec<(Id, Balance)>,
            _data: Vec<u8>,
        ) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp1155receiver {
    use crate::traits::{
        Id,
        PSP1155ReceiverError,
    };
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct PSP1155Receiver {}

    impl PSP1155Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "IPSP1155Receiver")]
    impl PSP1155Receiver {
        #[ink(message)]
        pub fn on_psp1155_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _id: Id,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP1155ReceiverError> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_psp1155_batch_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _ids_amounts: Vec<(Id, Balance)>,
            _data: Vec<u8>,
        ) -> Result<(), PSP1155ReceiverError> {
            unimplemented!()
        }
    }
}
