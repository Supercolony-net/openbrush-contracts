#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_access_control {
    use erc721::{
        traits::{ IErc721, Id, IErc721Mint },
        impls::{ Erc721Storage, Erc721, Erc721Mint }
    };
    use access_control::{
        traits::{ IAccessControl, RoleType },
        impls::{ AccessControlStorage, AccessControl, RoleData }
    };
    use brush::{
        traits::{ InkStorage },
        iml_getters,
    };
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
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

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc721Struct {
        // Fields of Erc721Storage
        /// Mapping from token to owner.
        token_owner: StorageHashMap<Id, AccountId>,
        /// Mapping from token to approvals users.
        token_approvals: StorageHashMap<Id, AccountId>,
        /// Mapping from owner to number of owned token.
        owned_tokens_count: StorageHashMap<AccountId, u32>,
        /// Mapping from owner to operator approvals.
        operator_approvals: StorageHashMap<(AccountId, AccountId), bool>,

        // Fields of AccessControlStorage
        /// Mapping from role type to role data(the list of members and admin role).
        roles: StorageHashMap<RoleType, RoleData>,
    }

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;

    impl Erc721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::_empty()
        }

        #[inline]
        fn only_minter(&self) {
            self._check_role(&MINTER, &self.env().caller());
        }
    }

    // We override _empty method and use it in the constructor.
    // _empty is a base constructor which can create an empty struct.
    // Some implementations require initialization of some variables, you can do it in _empty function.
    // In this case, all your constructors which are using _empty function will be initialized properly.
    impl InkStorage for Erc721Struct {
        fn _empty() -> Self {
            let mut instance = Self::default();
            let caller = Self::env().caller();
            instance._init_with_admin(caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance,MINTER, caller);
            instance
        }
    }
    impl Erc721Storage for Erc721Struct {
        iml_getters!(token_owner, _token_owner, _token_owner_mut, StorageHashMap<Id, AccountId>);
        iml_getters!(token_approvals, _token_approvals, _token_approvals_mut, StorageHashMap<Id, AccountId>);
        iml_getters!(owned_tokens_count, _owned_tokens_count, _owned_tokens_count_mut, StorageHashMap<AccountId, u32>);
        iml_getters!(operator_approvals, _operator_approvals, _operator_approvals_mut, StorageHashMap<(AccountId, AccountId), bool>);
    }
    impl AccessControlStorage for Erc721Struct {
        iml_getters!(roles, _roles, _roles_mut, StorageHashMap<RoleType, RoleData>);
    }

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

    impl_trait!(Erc721Struct, IErc721(Erc721), IAccessControl(AccessControl));

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
