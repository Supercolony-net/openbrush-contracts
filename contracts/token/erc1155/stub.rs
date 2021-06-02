use ink_lang as ink;
#[ink::contract(compile_as_dependency = true)]
pub mod erc1155receiver {
    use crate::traits::{Erc1155ReceiverError, Id};
    use ink_prelude::{vec::Vec};

    #[ink(storage)]
    pub struct Erc1155Receiver {}

    #[ink(namespace = "IErc1155Receiver")]
    impl Erc1155Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_erc1155_received(&mut self, _operator: AccountId, _from: AccountId,
                                   _id: Id, _value: Balance, _data: Vec<u8>) -> Result<(), Erc1155ReceiverError> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_erc1155_batch_received(&self, _operator: AccountId, _from: AccountId,
                                         _ids: Vec<Id>, _values: Vec<Balance>, _data: Vec<u8>) -> Result<(), Erc1155ReceiverError> {
            unimplemented!()
        }
    }
}