#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_token_timelock {
    use brush::contracts::psp22::utils::token_timelock::*;

    #[ink(storage)]
    #[derive(Default, PSP22TokenTimelockStorage)]
    pub struct MyPSP22TokenTimelock {
        #[PSP22TokenTimelockStorageField]
        timelock: PSP22TokenTimelockData,
    }

    impl PSP22TokenTimelock for MyPSP22TokenTimelock {}

    impl MyPSP22TokenTimelock {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            let mut instance = Self::default();
            assert!(instance._init(token_address, beneficiary, release_time).is_ok());
            instance
        }
    }
}
