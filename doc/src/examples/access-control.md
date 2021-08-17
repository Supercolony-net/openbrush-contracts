## Overview

This example shows how you can use the implementation of
[access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/access-control) and
[psp721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) together to provide rights to mint and burn NFT tokens.

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
   use brush::modifiers;
   use ink_prelude::vec::Vec;
```

3. Declare storage struct and declare the fields related to `PSP721Storage` and `AccessControlStorage`
   traits. Then you need to derive `PSP721Storage` and `AccessControlStorage` traits and mark according fields
   with `#[PSP721StorageField]` and `#[AccessControlStorageField]` attributes. Deriving these traits allow you to reuse
   the default implementation of `IPSP721` and `AccessControl`.

```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, AccessControlStorage)]
pub struct PSP721Struct {
    #[PSP721StorageField]
    psp721: PSP721Data,
    #[AccessControlStorageField]
    access: AccessControlData,
}
```

4. Inherit implementation of `IPSP721` and `AccessControl` traits. You can customize(override) methods in this `impl` block.

```rust
impl IPSP721 for PSP721Struct {}

impl AccessControl for PSP721Struct {}
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

6. Customize it by adding access control logic. We will implement `IPSP721Mint` trait. It will use modifier `only_minter`(it verifies that caller
   has minter role). Also, we need to update constructor to grant minter role to caller by default.

```rust
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
```
