#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37_enumerable {
    use openbrush::{
        contracts::psp37::extensions::{
            batch::*,
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data<enumerable::Balances>,
    }

    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {}

    impl PSP37Burnable for Contract {}

    impl PSP37Enumerable for Contract {}

    impl PSP37Batch for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp37::psp37_external::PSP37;
        use openbrush::contracts::psp37::extensions::enumerable::psp37enumerable_external::PSP37Enumerable;
        use openbrush::contracts::psp37::extensions::mintable::psp37mintable_external::PSP37Mintable;
        use openbrush::contracts::psp37::extensions::burnable::psp37burnable_external::PSP37Burnable;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn enumerable_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 2;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(token_by_index, Some(token_1.clone()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(token_by_index, Some(token_2.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, Some(token_1.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works_after_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 2;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(token_by_index, Some(token_1.clone()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(token_by_index, Some(token_2.clone()));

            let burn_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), vec![(token_2.clone(), amount_2)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("burn failed")
            }.return_value();

            assert_eq!(burn_tx, Ok(()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, Some(token_1.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_psp37_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, None);

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(token_by_index, Some(token_1.clone()));

            let token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(token_by_index, Some(token_2.clone()));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), token_2.clone(), amount_2, vec![]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }.return_value();

            assert_eq!(transfer_tx, Ok(()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, Some(token_1.clone()));

            let owners_token_by_index = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owners_token_by_index, Some(token_2.clone()));

            Ok(())
        }
    }
}
