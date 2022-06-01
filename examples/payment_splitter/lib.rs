#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_payment_splitter {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::payment_splitter::*;

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

        /// Payout all payees at once.
        /// Delete this method if you don't want this functionality in your version of the payment splitter.
        #[ink(message)]
        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {
            // `_release_all()` is an internal method defined by the `PaymentSplitterInternal` trait
            self._release_all()
        }
    }

    impl PaymentSplitter for SplitterStruct {}
}
