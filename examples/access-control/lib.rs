#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_access_control {
    use erc721::{
        traits::{ IErc721, Id, IErc721Mint },
        impls::{ Erc721Storage, Erc721, Erc721Mint, StorageHashMap }
    };
    use access_control::{
        traits::{ IAccessControl, RoleType },
        impls::{ AccessControlStorage, AccessControl, RoleData }
    };
    use brush::{
        traits::{ InkStorage },
    };
    use ink_lang::{ Env, EmitEvent };
    use ink_prelude::{ vec::Vec };

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when an operator is enabled or disabled for an owner.
    /// The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(storage)]
    #[derive(Default, Erc721Storage, AccessControlStorage, IErc721, IAccessControl)]
    pub struct Erc721Struct {}

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;

    impl Erc721Struct {
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
    impl InkStorage for Erc721Struct {}

    // Inheritance of Erc721 requires you to implement methods for event dispatching
    impl Erc721 for Erc721Struct {
        fn emit_transfer_event(&self, _from: AccountId, _to: AccountId, _id: Id) {
            self.env().emit_event(Transfer {
                from: Some(_from),
                to: Some(_to),
                id: _id,
            });
        }

        fn emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id) {
            self.env().emit_event(Approval {
                from: _from,
                to: _to,
                id: _id,
            });
        }

        fn emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner: _owner,
                operator: _operator,
                approved: _approved,
            });
        }
    }
    impl AccessControl for Erc721Struct {}

    impl Erc721Mint for Erc721Struct {}
    impl IErc721Mint for Erc721Struct {
        #[ink(message)]
        fn mint(&mut self, id: Id) {
            self.only_minter();
            Erc721Mint::mint(self, id);
        }

        #[ink(message)]
        fn burn(&mut self, id: Id) {
            self.only_minter();
            Erc721Mint::burn(self, id);
        }
    }
}
