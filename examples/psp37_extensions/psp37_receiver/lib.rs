#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp37::*,
        storage::Mapping,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl PSP37Receiver for Contract {
        #[ink(message)]
        fn before_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
            data: Vec<u8>,
        ) -> Result<(), PSP37ReceiverError> {
            Ok(())
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
