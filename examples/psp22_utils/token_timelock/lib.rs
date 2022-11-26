#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::{
        contracts::psp22::utils::token_timelock::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        timelock: token_timelock::Data,
    }

    impl PSP22TokenTimelock for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            ink::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._init(token_address, beneficiary, release_time).is_ok());
            })
        }
    }
}
