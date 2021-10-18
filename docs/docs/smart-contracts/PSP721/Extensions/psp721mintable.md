---
sidebar_position: 1
title: PSP721 Mintable
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/burnable.rs) extension.

## Step 1: Include dependencies

Include dependencies on `psp721` and `brush` in the cargo file.

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
psp721 = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
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
   "psp721/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `psp721::traits` and also `psp721::extensions::mintable`.

```rust
#[brush::contract]
pub mod my_psp721 {
   use psp721::{
        extensions::{
            mintable::*,
        },
        traits::*,
    };
```

## Step 3: Define storage

Declare storage struct and declare the field related to `PSP721Storage`trait. Then you need to derive `PSP721Storage` trait and mark corresponding field with `#[PSP721StorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `PSP721` to which we will add an implementation of our `PSP721Mintable` extension.

```rust
#[ink(storage)]
#[derive(Default, PSP721Storage)]
pub struct MyPSP721 {
    #[PSP721StorageField]
    psp721: PSP721Data,
}
```

## Step 4: Inherit logic

Inherit implementations of `PSP721` and `PSP721Mintable` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP721 for MyPSP721 {}

impl PSP721Mintable for MyPSP721 {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `PSP721Mintable` contract is ready!

```rust
impl MyPSP721 {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```

## Step 6: Customize your contract

Customize it by adding `#[ink(message)]` annotated functions to your contract implementation, or to implementation of the extension, or override the original functions in the implementation.
