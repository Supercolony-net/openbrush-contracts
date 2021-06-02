use ink_lang as ink;

#[ink::contract(compile_as_dependency = true)]
pub mod erc721receiver {
    use crate::traits::{Erc721ReceiverError, Id};
    use ink_prelude::{vec::Vec};

    #[ink(storage)]
    pub struct Erc721Receiver {}

    #[ink(namespace = "IErc721Receiver")]
    impl Erc721Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_erc721_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            id: Id,
            data: Vec<u8>,
        ) -> Result<(), Erc721ReceiverError> {
            unimplemented!()
        }
    }
}