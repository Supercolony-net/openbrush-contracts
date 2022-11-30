#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod diamond {
    use openbrush::{
        contracts::diamond::extensions::diamond_loupe::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        diamond: diamond::Data<Loupe>,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(owner);

            instance
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            self._fallback()
        }
    }

    impl Ownable for Contract {}

    impl Diamond for Contract {}

    impl DiamondLoupe for Contract {}
}
