## Overview
This example shows how you can use the implementation of
[access-control](contracts/access/access-control) and 
[erc721](contracts/token/erc721) together to provide rights 
to mint and burn NFT tokens.

## Steps
1. You need to include `erc721`, `access-control` and `brush` in cargo file.
```markdown
[dependencies]
...

erc721 = { version = "0.2.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
access-control = { version = "0.2.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
brush = { version = "0.2.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "erc721/std",
   "access-control/std",
   "brush/std",
]
```
2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`. 
Import traits, errors, macros and structs which you want to use.
```rust
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
```
3. Declare storage struct that will contain all fields for 
`AccessControlStorage` and `Erc721Storage` traits.
Declare events(example of events you can find in tests of [Erc721](contracts/token/erc721/impls.rs))

```rust
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
```
4. Implement storage traits by using `iml_getters` macro.
```rust
impl InkStorage for Erc721Struct {}
impl Erc721Storage for Erc721Struct {
    iml_getters!(token_owner, _token_owner, _token_owner_mut, StorageHashMap<Id, AccountId>);
    iml_getters!(token_approvals, _token_approvals, _token_approvals_mut, StorageHashMap<Id, AccountId>);
    iml_getters!(owned_tokens_count, _owned_tokens_count, _owned_tokens_count_mut, StorageHashMap<AccountId, u32>);
    iml_getters!(operator_approvals, _operator_approvals, _operator_approvals_mut, StorageHashMap<(AccountId, AccountId), bool>);
}
impl AccessControlStorage for Erc721Struct {
    iml_getters!(roles, _roles, _roles_mut, StorageHashMap<RoleType, RoleData>);
}
```
5. After that you can inherit implementation of `Erc721` and `AccessControl` traits.
```rust
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
```
6. Now you have all basic logic of `Erc721` and `AccessControl` on rust level.
But all methods are internal now(it means that anyone can't call these methods from outside of contract). 
If you want to make them external you MUST implement `IErc721` and `IAccessControl` traits.
Library provides macro `impl_trait` that will generate external implementation of all methods from `IErc721` and `IAccessControl` traits.
Macro will call the methods with the same name from `Erc721` and `AccessControl` traits.
```rust
impl_trait!(Erc721Struct, IErc721(Erc721), IAccessControl(AccessControl));
```
7. Now you only need to define constructor and your basic version of `Erc721` contract is ready.
```rust
impl Erc721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::_empty()
    }
}
// We override _empty method and use it in the constructor. 
// _empty is a base constructor which can create an empty struct. 
// Some implementations require initialization of some variables, you can do it in _empty function. 
// In this case, all your constructors which are using _empty function will be initialized properly.
impl InkStorage for Erc721Struct {
    fn _empty() -> Self {
        let mut instance = Self::default();
        instance._init_with_admin(Self::env().caller());
        instance
    }
}
```
8. Let's customize it. We will implement `IErc721Mint` trait. For that we need inherit `Erc721Mint`. 
It will call `only_minter` function inside to verify that caller has minter role.
Also, we need to update `_empty` function to grant minter role to caller by default.
```rust
// ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
const MINTER: RoleType = 0xfd9ab216;

impl Erc721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::_empty()
    }

    #[inline]
    fn only_minter(&self) -> Result<(), Erc721Error> {
        if !self.has_role(MINTER, self.env().caller()) {
            return Err(Erc721Error::Unknown("Caller is not minter".to_string()));
        }

        Ok(())
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
        instance._grant_role(MINTER, caller).expect("Can't provide Minter to caller");
        instance
    }
}

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
```
