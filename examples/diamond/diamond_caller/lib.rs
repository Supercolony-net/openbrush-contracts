#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod diamond_caller {
    use brush::contracts::psp22::*;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    /// The purpose of this contract is to call a function of a facet of diamond standard contract
    pub struct DiamondCaller {}

    impl DiamondCaller {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_: &mut DiamondCaller| {})
        }

        #[ink(message)]
        pub fn balance_of(&mut self, token: AccountId, account: AccountId) -> Balance {
            PSP22Ref::balance_of(&token, account)
        }
    }
}
