#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_mintable {
    use openbrush::{
        contracts::psp34::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Mintable for Contract {}

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::extensions::mintable::psp34mintable_external::PSP34Mintable;
        use openbrush::contracts::psp34::psp34_external::PSP34;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp34_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 0);
            assert_eq!(balance_of!(client, address, bob), 0);

            let id_1 = Id::U8(1);
            let id_2 = Id::U8(2);

            let mint_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), id_1.clone()));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), id_2.clone()));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_2, Ok(()));

            assert_eq!(balance_of!(client, address, alice), 1);
            assert_eq!(balance_of!(client, address, bob), 1);

            Ok(())
        }

        #[ink_e2e::test]
        async fn mint_existing_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp34_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 0);
            assert_eq!(balance_of!(client, address, bob), 0);

            let id_1 = Id::U8(1);

            let mint_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), id_1.clone()));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), id_1.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(mint_2, Err(_)));

            assert_eq!(balance_of!(client, address, alice), 1);
            assert_eq!(balance_of!(client, address, bob), 0);

            Ok(())
        }
    }
}
