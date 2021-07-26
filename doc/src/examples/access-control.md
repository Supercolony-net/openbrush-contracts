## Overview
This example shows how you can use the implementation of
**access-control** and **psp721** together to provide rights to mint and burn NFT tokens.

## Steps
1. Include dependencies `psp721`, `access-control` and `brush` in cargo file.
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
2. Replace `ink::contract` macro by `brush::contract`.
   Import **everything** from according trait modules.
```rust
#[brush::contract]
pub mod my_access_control {
   use psp721::traits::*;
   use access_control::traits::*;
```
3. Declare storage struct and derive `PSP721Storage` and `AccessControlStorage`
   traits. Deriving these traits will add required fields to your structure
   for implementation of according traits. Your structure must implement
   `PSP721Storage` and `AccessControlStorage` traits if you want to use the
   default implementation of `IPSP721` and `IAccessControl`.
```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, AccessControlStorage)]
pub struct PSP721Struct {}
```
4. Inherit implementation of `IPSP721` and `IAccessControl` traits.
   You can customize(override) methods in this `impl` block.
```rust
impl IPSP721 for PSP721Struct {}
impl IAccessControl for PSP721Struct {}
```
5. Define constructor and your basic version of `IPSP721` contract is ready.
```rust
impl PSP721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```
6. Customize it by adding access control logic. We will implement `IPSP721Mint` trait.
   It will use modifier `only_minter`(it verifies that caller has minter role).
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
      instance.grant_role(MINTER, caller);
      instance
   }

   fn only_minter(&self) {
      self._check_role(&MINTER, &self.env().caller());
   }
}

impl IPSP721Mint for PSP721Struct {
   #[modifiers(only_minter)]
   fn mint(&mut self, id: Id) {
      // We added modifier to function. 
      // #[super]self.mint(id) will call default implementation from trait
      #[super]self.mint(id);
   }

   #[modifiers(only_minter)]
   fn burn(&mut self, id: Id) {
      // We added modifier to function. 
      // #[super]self.burn(id) will call default implementation from trait
      #[super]self.burn(id);
   }
}
```
