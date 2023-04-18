#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod ownable {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            ownable::*,
            psp37::extensions::{
                burnable::*,
                mintable::*,
            },
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance
        }
    }

    impl Ownable for Contract {}

    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            self._mint_to(to, ids_amounts)
        }
    }

    impl PSP37Burnable for Contract {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            self._burn_from(from, ids_amounts)
        }
    }


    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::ownable::ownable_external::Ownable;
        use openbrush::contracts::psp37::psp37_external::PSP37;
        use openbrush::contracts::psp37::extensions::mintable::psp37mintable_external::PSP37Mintable;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn owner_is_by_default_contract_deployer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_owner_is_allowed_to_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), vec![(Id::U8(0), 1)]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_ownership_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token = Id::U8(1);
            let ids_amounts = vec![(token.clone(), 123)];

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), ids_amounts.clone()));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(mint_tx, Err(_)));

            let balance_before = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(bob), Some(token.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(balance_before, 0);

            let transfer_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer_ownership(address_of!(bob)));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer_ownership failed")
            }.return_value();

            assert_eq!(transfer_ownership_tx, Ok(()));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(bob));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), ids_amounts.clone()));
                client.call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }.return_value();

            assert_eq!(mint_tx, Ok(()));

            let balance_after = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(bob), Some(token.clone())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(balance_after, 123);

            Ok(())
        }

        #[ink_e2e::test]
        async fn renounce_ownership_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            let renounce_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_ownership());
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("renounce_ownership failed")
            }.return_value();

            assert_eq!(renounce_ownership_tx, Ok(()));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, AccountId::from([0x00; 32]));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_renounce_ownership_if_not_owner(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            let renounce_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_ownership());
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(renounce_ownership_tx, Err(_)));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_ownership_if_not_owner(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_ownable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            let renounce_ownership_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_ownership());
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(renounce_ownership_tx, Err(_)));

            let owner = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner, address_of!(alice));

            Ok(())
        }
    }
}
