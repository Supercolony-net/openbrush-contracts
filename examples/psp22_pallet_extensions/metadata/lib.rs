#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet_metadata {
    use openbrush::{
        contracts::psp22_pallet::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}

    impl PSP22Metadata for Contract {}

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(
            asset_id: u32,
            min_balance: Balance,
            total_supply: Balance,
            name: String,
            symbol: String,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();

            instance
                ._create(asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id = asset_id;
            instance.pallet.origin = Origin::Caller;
            assert!(instance
                .pallet
                .pallet_assets
                .set_metadata(asset_id, name, symbol, decimal)
                .is_ok());
            instance
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
    }
}
