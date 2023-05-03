#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        contracts::access_control::extensions::enumerable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access: access_control::Data<enumerable::Members>,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl AccessControl for Contract {}

    impl AccessControlEnumerable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            instance._init_with_admin(caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            instance.grant_role(MINTER, caller).expect("Should grant MINTER role");
            assert_eq!(instance.get_role_member_count(MINTER), 1);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::access_control::{
            accesscontrol_external::AccessControl,
            extensions::enumerable::accesscontrolenumerable_external::AccessControlEnumerable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            get_role_member,
            get_role_member_count,
            grant_role,
            has_role,
            revoke_role,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn should_have_not_member(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(get_role_member!(client, address, MINTER, 1), None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_get_role_member(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(get_role_member!(client, address, MINTER, 0), Some(address_of!(alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_roles_and_get_role_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(grant_role!(client, address, MINTER, bob), Ok(()));

            assert_eq!(get_role_member!(client, address, MINTER, 1), Some(address_of!(bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoker_and_count_roles(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control_enumerable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, bob), false);

            assert_eq!(has_role!(client, address, MINTER, alice), true);

            assert_eq!(get_role_member_count!(client, address, MINTER), 1);

            assert_eq!(revoke_role!(client, address, MINTER, alice), Ok(()));

            assert_eq!(get_role_member_count!(client, address, MINTER), 0);

            Ok(())
        }
    }
}
