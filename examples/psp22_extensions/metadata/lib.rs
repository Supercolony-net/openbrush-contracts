#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for Contract {}

    impl PSP22Metadata for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimal;
            instance
                ._mint_to(instance.env().caller(), total_supply)
                .expect("Should mint total_supply");

            instance
        }
    }
}
