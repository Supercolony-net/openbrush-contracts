#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_payment_splitter {
    use payment_splitter::traits::*;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, PaymentSplitterStorage)]
    pub struct SplitterStruct {
        #[PaymentSplitterStorageField]
        splitter: PaymentSplitterData,
    }

    impl SplitterStruct {
        #[ink(constructor)]
        pub fn new(payees: Vec<AccountId>, shares: Vec<Balance>) -> Self {
            let mut instance = Self::default();
            instance._init(payees, shares);
            instance
        }
    }

    impl PaymentSplitter for SplitterStruct {}
}
