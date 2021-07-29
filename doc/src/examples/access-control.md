## Overview
This example shows how you can use the implementation of
[access-control](contracts/access/access-control) and 
[psp721](contracts/token/psp721) together to provide rights 
to mint and burn NFT tokens.

## Steps
1. You need to include `psp721`, `access-control` and `brush` in cargo file.
```markdown
[dependencies]
...

psp721 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
access-control = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "psp721/std",
   "access-control/std",
   "brush/std",
]
```
2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`. 
Import traits, errors, macros and structs which you want to use.
```rust
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
   use ink_prelude::{ vec::Vec };
```
3. Declare storage struct and derive `PSP721Storage` and `AccessControlStorage` 
   traits. Deriving these traits will add required fields to your structure 
   for implementation of according traits. Your structure must implement 
   `PSP721Storage` and `AccessControlStorage` traits if you want to use the 
   default implementation of `PSP721` and `AccessControl`.
```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, AccessControlStorage)]
pub struct PSP721Struct {}
```
4. After that you can inherit implementation of `PSP721` and `AccessControl` traits.
You can customize(override) some methods there.
```rust
// InkStorage is a utils trait required by any Storage trait
impl PSP721 for PSP721Struct {}
impl AccessControl for PSP721Struct {}
```
5. Now you have all basic logic of `PSP721` and `AccessControl` on rust level.
But all methods are internal now(it means that anyone can't call these methods from outside of contract). 
If you want to make them external you MUST derive `IPSP721` and `IAccessControl` traits.
Deriving of these traits will generate external implementation of all methods from `IPSP721` and `IAccessControl`.
Macro will call the methods with the same name from `PSP721` and `AccessControl` traits.
```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, AccessControlStorage, IPSP721, IAccessControl)]
pub struct PSP721Struct {}
```
6. Now you only need to define constructor and your basic version of `PSP721` contract is ready.
```rust
impl PSP721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```
7. Let's customize it. We will implement `IPSP721Mint` trait. For that we need inherit `PSP721Mint`. 
It will call `only_minter` function inside to verify that caller has minter role.
Also, we need to update constructor to grant minter role to caller by default.
```rust
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
```
