#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22_pallet::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {
                // The contract is admin of the asset
                instance
                    ._create(asset_id, Self::env().account_id(), min_balance)
                    .expect("Should create an asset");
                instance.pallet.asset_id = asset_id;
                instance.pallet.origin = Origin::Caller;
                instance
                    ._mint_to(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }

        /// Asset id of the asset in the `pallet-assets`
        #[ink(message)]
        pub fn asset_id(&self) -> u32 {
            self.pallet.asset_id
        }
    }
}
