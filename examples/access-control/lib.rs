#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_access_control {
    use psp721::traits::*;
    use access_control::traits::*;
    use brush::modifiers;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, PSP721Storage, AccessControlStorage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[AccessControlStorageField]
        access: AccessControlData,
    }

    const MINTER: RoleType = brush::blake2b_256_as_u32!("MINTER");

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            instance.grant_role(MINTER, caller);
            instance
        }
    }

    impl IPSP721 for PSP721Struct {}

    impl AccessControl for PSP721Struct {}

    impl IPSP721Mint for PSP721Struct {
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn mint(&mut self, id: Id) {
            self._mint(id);
        }

        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        fn burn(&mut self, id: Id) {
            self._burn(id);
        }
    }
}
