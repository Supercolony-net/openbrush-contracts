## Overview

This example shows how you can use the implementation of
[access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/access-control) and
[psp721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) together to provide rights to mint and burn NFT tokens.

## Steps

1. Include dependencies to `psp721`, `access-control` and `brush` in the cargo file.

```markdown
[dependencies]
ink_primitives = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }
scale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }

# These dependencies
psp721 = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
access-control = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
   "ink_primitives/std",
   "ink_metadata",
   "ink_metadata/std",
   "ink_env/std",
   "ink_storage/std",
   "ink_lang/std",
   "scale/std",
   "scale-info",
   "scale-info/std",

   # These dependencies   
   "psp721/std",
   "access-control/std",
   "brush/std",
]
```

2. Replace `ink::contract` macro by `brush::contract`.
   Import **everything** from `psp721::traits` and `access_control::traits`.

```rust
#[brush::contract]
pub mod my_access_control {
   use psp721::traits::*;
   use access_control::traits::*;
   use brush::modifiers;
   use ink_prelude::vec::Vec;
```

3. Declare storage struct and declare the fields related to `PSP721Storage` and `AccessControlStorage`
   traits. Then you need to derive `PSP721Storage` and `AccessControlStorage` traits and mark corresponsing fields
   with `#[PSP721StorageField]` and `#[AccessControlStorageField]` attributes. Deriving these traits allows you to reuse
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

4. Inherit implementations of `IPSP721` and `AccessControl` traits. You can customize (override) methods in this `impl` block.

```rust
impl IPSP721 for PSP721Struct {}

impl AccessControl for PSP721Struct {}
```

5. Define constructor. Your basic version of `IPSP721` contract is ready!

```rust
impl PSP721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```

6. Customize it by adding access control logic. We will implement `IPSP721Mint` trait. It will use modifier `only_minter`(it verifies that caller
   has the minter role). Also, we need to update the constructor to grant the minter role to the caller by default.

```rust
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
