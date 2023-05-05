#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_burnable {
    use openbrush::{
        contracts::psp34::extensions::burnable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Burnable for Contract {}

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            instance
                ._mint_to(Self::env().caller(), Id::U8(0u8))
                .expect("Should mint token with id 0");
            instance
                ._mint_to(Self::env().caller(), Id::U8(1u8))
                .expect("Should mint token with id 1");
            instance
                ._mint_to(Self::env().caller(), Id::U8(2u8))
                .expect("Should mint token with id 2");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::{
            extensions::burnable::psp34burnable_external::PSP34Burnable,
            psp34_external::PSP34,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of,
            owner_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn burn_wokrs(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 3);

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), Id::U8(0u8)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert_eq!(result.return_value(), Ok(()));
            assert_eq!(balance_of!(client, address, alice), 2);

            Ok(())
        }

        #[ink_e2e::test]
        async fn burn_from_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp34_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 3);

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), Id::U8(0u8)));
                client.call(&ink_e2e::bob(), _msg, 0, None).await.expect("call failed")
            };

            assert_eq!(result.return_value(), Ok(()));
            assert_eq!(balance_of!(client, address, alice), 2);

            Ok(())
        }
    }
}
