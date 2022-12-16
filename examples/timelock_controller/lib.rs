#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_timelock_controller {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::timelock_controller::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            access_control::Internal::_init_with_admin(&mut instance, caller);
            timelock_controller::Internal::_init_with_admin(&mut instance, caller, min_delay, proposers, executors);

            instance
        }
    }

    // `TimelockController` is an extension for `AccessControl`, so you have to inherit logic related to both modules.
    impl AccessControl for Contract {}
    impl TimelockController for Contract {}
}
