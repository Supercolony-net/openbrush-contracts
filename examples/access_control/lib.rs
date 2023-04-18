#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        contracts::{
            access_control::*,
            psp34::extensions::{
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
        psp34: psp34::Data,
        #[storage_field]
        access: access_control::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            instance.grant_role(MINTER, caller).expect("Should grant MINTER role");

            instance
        }
    }

    impl PSP34 for Contract {}

    impl AccessControl for Contract {}

    impl PSP34Mintable for Contract {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(account, id)
        }
    }

    impl PSP34Burnable for Contract {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._burn_from(account, id)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::access_control::accesscontrol_external::AccessControl;
        use openbrush::contracts::psp34::extensions::mintable::psp34mintable_external::PSP34Mintable;
        use openbrush::contracts::psp34::psp34_external::PSP34;
        use openbrush::contracts::psp34::extensions::burnable::psp34burnable_external::PSP34Burnable;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use openbrush::contracts::access_control::DEFAULT_ADMIN_ROLE;

        use test_helpers::{
            address_of,
            has_role,
            grant_role,
            mint_dry_run,
            mint,
            revoke_role,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn only_minter_role_is_allowed_to_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;


            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert!(matches!(mint_dry_run!(client, address, bob, bob, Id::U8(0)), Err(_)));

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);

            assert_eq!(mint!(client, address, bob, bob, Id::U8(0)), Ok(()));

            let owner_of = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner_of(Id::U8(0)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner_of, Some(address_of!(bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_initial_roles_to_default_signer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, alice), true);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_grant_initial_roles_for_random_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_change_old_roles_after_grant_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, alice), true);
            assert_eq!(has_role!(client, address, MINTER, alice), true);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, alice), true);
            assert_eq!(has_role!(client, address, MINTER, alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoke_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), true);

            let revoke_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.revoke_role(MINTER, address_of!(bob)));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }.return_value();

            assert_eq!(revoke_role, Ok(()));

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_renounce_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, alice), true);

            let renounce_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_role(MINTER, address_of!(alice)));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }.return_value();

            assert_eq!(renounce_role, Ok(()));

            assert_eq!(has_role!(client, address, MINTER, alice), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_grant_or_revoke_not_by_admin_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            let grant_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.grant_role(MINTER, address_of!(charlie)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(grant_role, Err(_)));

            let revoke_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.revoke_role(MINTER, address_of!(charlie)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(revoke_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_renounce_not_self_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, bob), true);

            let renounce_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_role(MINTER, address_of!(bob)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(renounce_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_burn_if_no_minter_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client.instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // await tx.grantRole(Roles.Minter, alice.address)
            // await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
            // await contract.withSigner(alice).tx.mint(alice.address, IdBuilder.U8(0))
            // await expect(query.ownerOf(IdBuilder.U8(0))).to.have.output(alice.address)
            //
            // // Act - revoke Alice minter role
            // await tx.revokeRole(Roles.Minter, alice.address)
            // await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
            //
            // // Assert - Alice cannot burn token
            // await expect(contract.withSigner(alice).tx.burn(alice.address, IdBuilder.U8(0))).to.eventually.be.rejected

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, bob), true);

            assert_eq!(mint!(client, address, bob, bob, Id::U8(0)), Ok(()));

            let owner_of = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owner_of(Id::U8(0)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(owner_of, Some(address_of!(bob)));

            assert_eq!(revoke_role!(client, address, MINTER, bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, bob), false);

            let burn = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), Id::U8(0)));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(burn, Err(_)));

            Ok(())
        }
    }
}