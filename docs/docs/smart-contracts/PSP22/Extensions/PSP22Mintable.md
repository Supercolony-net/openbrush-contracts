---
sidebar_position: 1
title: PSP22Mintable
---

This example shows how you can reuse the implementation of
[psp22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/mintable.rs) extension.

## Step 1: Include dependencies

Include dependencies to `psp22` and `brush` in the cargo file.

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
psp22 = { path = "../../contracts/token/psp22", default-features = false }
brush = { path = "../../utils/brush", default-features = false }

[lib]
name = "my_psp22"
path = "lib.rs"
crate-type = [
    # Used for normal contract Wasm blobs.
    "cdylib",
]

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
    "psp22/std",
    "brush/std",
]
ink-as-dependency = []

```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `psp22::extensions::mintable::*`.

```rust
#[brush::contract]
pub mod my_psp22 {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::mintable::*,
        traits::*,
    };
```

## Step 3: Define storage

Declare storage struct and declare the fields related to `PSP22Storage` trait. 
Then you need to derive `PSP22Storage` traits and mark corresponding fields
with `#[PSP22StorageField]` attribute. Deriving this trait allows you to reuse
the default implementation of `PSP22`.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
}
```

## Step 4: Inherit logic

Inherit implementations of `PSP22` and `PSP22Mintable` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP22 for MyPSP22 {}

impl PSP22Mintable for MyPSP22 {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `PSP22` contract with 'PSP22Mintable' extension is ready!

```rust
impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance) -> Self {
        let mut instance = Self::default();
        instance._mint(instance.env().caller(), _total_supply);
        instance
    }
}
```

## Step 6: Customize your contract

Customize it by adding functionality of minting to account.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
}

impl PSP22 for MyPSP22 {}

impl PSP22Mintable for MyPSP22 {}

impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance) -> Self {
        let mut instance = Self::default();
        instance._mint(instance.env().caller(), _total_supply);
        instance
    }

    #[ink(message)]
    pub fn mint_to(&mut self, account: AccountId, amount: Balance) {
        self.mint(account, amount);
    }
}


```
