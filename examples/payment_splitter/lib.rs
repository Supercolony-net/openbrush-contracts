#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_payment_splitter {
    use brush::contracts::payment_splitter::*;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PaymentSplitterStorage)]
    pub struct SplitterStruct {
        #[PaymentSplitterStorageField]
        splitter: PaymentSplitterData,
    }

    impl SplitterStruct {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init(payees_and_shares).expect("Should init");
            })
        }
    }

    impl PaymentSplitter for SplitterStruct {}
}
