---
sidebar_position: 2
title: PSP1155 Mintable
---

This example shows how you can reuse the implementation of [PSP1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155) token with [PSP1155Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155/src/extensions/mintable.rs) extension.

## Step 1: Include dependencies

Include dependencies on `psp1155` and `brush` in the cargo file.

```toml
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
psp1155 = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

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
   "psp1155/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `psp1155::traits` and also `psp1155::extensions::mintable`.

```rust
#[brush::contract]
pub mod my_psp1155 {
   use psp1155::{
        extensions::{
            mintable::*,
        },
        traits::*,
    };
```

## Step 3: Define storage

Declare storage struct and declare the field related to `PSP1155Storage`trait. Then you need to derive `PSP1155Storage` trait and mark corresponding field with `#[PSP1155StorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `PSP1155` to which we will add an implementation of our `PSP1155Mintable` extension.

```rust
#[ink(storage)]
#[derive(Default, PSP1155Storage)]
pub struct MyPSP1155 {
    #[PSP1155StorageField]
    psp1155: PSP1155Data,
}
```

## Step 4: Inherit logic

Inherit implementations of `PSP1155` and `PSP1155Mintable` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP1155 for MyPSP1155 {}

impl PSP1155Mintable for MyPSP1155 {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `PSP1155Mintable` contract is ready!

```rust
impl MyPSP1155 {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```

## Step 6: Customize your contract

Customize it by adding `#[ink(message)]` annotated functions to your contract implementation, or to implementation of the extension, or override the original functions in the implementation.
