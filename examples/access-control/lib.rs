#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_access_control {
    use psp721::{
        traits::{ IPSP721, Id, IPSP721Mint },
        impls::{ PSP721Storage, PSP721, PSP721Mint, StorageHashMap }
    };
    use access_control::{
        traits::{ IAccessControl, RoleType },
        impls::{ AccessControlStorage, AccessControl, RoleData }
    };
    use brush::{
        traits::{ InkStorage },
    };
    use ink_prelude::{ vec::Vec };

    #[ink(storage)]
    #[derive(Default, PSP721Storage, AccessControlStorage, IPSP721, IAccessControl)]
    pub struct PSP721Struct {}

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance,MINTER, caller);
            instance
        }

        #[inline]
        fn only_minter(&self) {
            self._check_role(&MINTER, &self.env().caller());
        }
    }

    // InkStorage is a utils trait required by any Storage trait
    impl InkStorage for PSP721Struct {}
    impl PSP721 for PSP721Struct {}
    impl AccessControl for PSP721Struct {}

    impl PSP721Mint for PSP721Struct {}
    impl IPSP721Mint for PSP721Struct {
        #[ink(message)]
        fn mint(&mut self, id: Id) {
            self.only_minter();
            PSP721Mint::mint(self, id);
        }

        #[ink(message)]
        fn burn(&mut self, id: Id) {
            self.only_minter();
            PSP721Mint::burn(self, id);
        }
    }
}
