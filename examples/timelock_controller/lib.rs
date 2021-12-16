#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_timelock_controller {
    use brush::contracts::timelock_controller::*;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, AccessControlStorage, TimelockControllerStorage)]
    pub struct TimelockStruct {
        #[AccessControlStorageField]
        access: AccessControlData,
        #[TimelockControllerStorageField]
        timelock: TimelockControllerData,
    }

    impl TimelockStruct {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            AccessControlInternal::_init_with_admin(&mut instance, caller);
            TimelockControllerInternal::_init_with_admin(&mut instance, caller, min_delay, proposers, executors);
            instance
        }
    }

    // `TimelockController` is an extension for `AccessControl`, so you have to inherit logic related to both modules.
    impl AccessControl for TimelockStruct {}
    impl TimelockController for TimelockStruct {}
}
