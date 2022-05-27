#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            access_control::*,
            psp34::extensions::{
                burnable::*,
                mintable::*,
            },
        },
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP34Storage, AccessControlStorage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[AccessControlStorageField]
        access: AccessControlData,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink_lang::selector_id!("MINTER");

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_admin(caller);
                // We grant minter role to caller in constructor, so he can mint/burn tokens
                instance.grant_role(MINTER, caller).expect("Should grant MINTER role");
            })
        }
    }

    impl PSP34 for PSP34Struct {}

    impl AccessControl for PSP34Struct {}

    impl PSP34Mintable for PSP34Struct {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(account, id)
        }
    }

    impl PSP34Burnable for PSP34Struct {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._burn_from(account, id)
        }
    }
}
