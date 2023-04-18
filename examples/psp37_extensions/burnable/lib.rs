#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp37::extensions::burnable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl PSP37 for Contract {}

    impl PSP37Burnable for Contract {}

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            self._mint_to(to, ids_amounts)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp37::psp37_external::PSP37;
        use openbrush::contracts::psp37::extensions::burnable::psp37burnable_external::PSP37Burnable;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of_37
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn burn_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint_to(address_of!(alice), vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer_from(address_of!(alice), address_of!(bob), token_1.clone(), amount_1.clone(), vec![]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }.return_value();

            assert_eq!(transfer_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, None), 1);
            assert_eq!(balance_of_37!(client, address, bob, None), 1);

            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), amount_2);

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(None));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply, 2);

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), vec![(token_2.clone(), amount_2.clone())]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("burn failed")
            }.return_value();

            assert_eq!(burn_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, None), 0);
            assert_eq!(balance_of_37!(client, address, bob, None), 1);

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(None));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply, 1);

            let total_supply_token_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(Some(token_1.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply_token_1, 1);

            let total_supply_token_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(Some(token_2.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply_token_2, 0);

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), vec![(token_1.clone(), amount_1.clone())]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("burn failed")
            }.return_value();

            assert_eq!(burn_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, None), 0);
            assert_eq!(balance_of_37!(client, address, bob, None), 0);

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(None));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply, 0);

            let total_supply_token_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(Some(token_1.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply_token_1, 0);

            let total_supply_token_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply(Some(token_2.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(total_supply_token_2, 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn burn_batch_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 10;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint_to(address_of!(alice), vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), 20)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer_from(address_of!(alice), address_of!(bob), token_2.clone(), amount_2.clone(), vec![]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }.return_value();

            assert_eq!(transfer_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), amount_2);

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("burn failed")
            }.return_value();

            assert_eq!(burn_tx, Ok(()));

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), vec![(token_1.clone(), 0), (token_2.clone(), amount_2.clone())]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("burn failed")
            }.return_value();

            assert_eq!(burn_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), 0);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn burn_insufficient_balance_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 10;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint_to(address_of!(alice), vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), 0);

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), vec![(token_1.clone(), amount_1 + 1), (token_2.clone(), amount_2.clone())]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(burn_tx, Err(_)));

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), vec![(token_1.clone(), amount_1 + 1)]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(burn_tx, Err(_)));

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), vec![(token_1.clone(), amount_1 + 1), (token_2.clone(), amount_2.clone())]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(burn_tx, Err(_)));

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), vec![(token_1.clone(), amount_1 + 1)]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(burn_tx, Err(_)));

            assert_eq!(balance_of_37!(client, address, alice, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, alice, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, bob, Some(token_2.clone())), 0);

            Ok(())
        }
    }
}
