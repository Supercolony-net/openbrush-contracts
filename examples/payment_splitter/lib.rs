#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_payment_splitter {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::payment_splitter::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        splitter: payment_splitter::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            let mut instance = Self::default();
            instance._init(payees_and_shares).expect("Should init");
            instance
        }

        /// Payout all payees at once.
        /// Delete this method if you don't want this functionality in your version of the payment splitter.
        #[ink(message)]
        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {
            // `_release_all()` is an internal method defined by the `payment_splitter::Internal` trait
            self._release_all()
        }
    }

    impl PaymentSplitter for Contract {}
}
