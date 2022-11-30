#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod diamond_caller {
    use ink::prelude::vec::Vec;
    use openbrush::contracts::psp22::*;

    #[ink(storage)]
    #[derive(Default)]
    /// The purpose of this contract is to call a function of a facet of diamond standard contract
    pub struct DiamondCaller {}

    impl DiamondCaller {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn balance_of(&mut self, token: AccountId, account: AccountId) -> Balance {
            PSP22Ref::balance_of(&token, account)
        }

        #[ink(message)]
        pub fn transfer(&mut self, token: AccountId, to: AccountId, value: Balance) -> Result<(), PSP22Error> {
            PSP22Ref::transfer_from(&token, Self::env().caller(), to, value, Vec::<u8>::new())
        }
    }
}
