#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_access_control {
    use access_control::traits::*;
    use brush::modifiers;
    use ink_prelude::vec::Vec;
    use psp721::{
        extensions::{
            burnable::*,
            mintable::*,
        },
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP721Storage, AccessControlStorage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[AccessControlStorageField]
        access: AccessControlData,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = brush::blake2b_256_as_u32!("MINTER");

    impl PSP721Struct {
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

    impl PSP721 for PSP721Struct {}

    impl AccessControl for PSP721Struct {}

    impl PSP721Mintable for PSP721Struct {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn mint(&mut self, id: Id) -> Result<(), PSP721Error> {
            self._mint(id)
        }

        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn mint_to(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
            self._mint_to(account, id)
        }
    }

    impl PSP721Burnable for PSP721Struct {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn burn(&mut self, id: Id) -> Result<(), PSP721Error> {
            self._burn(id)
        }

        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn burn_from(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
            self._burn_from(account, id)
        }
    }
}
