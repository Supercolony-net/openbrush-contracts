---
sidebar_position: 6
title: PSP22 Pausable
---

This example shows how you can implement a [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) contract with a [Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable) extension. See an example of [PSP22Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/pausable) implementation.

## Step 1: Include dependencies

Include dependencies to `psp22`, `pausable` and `brush` in the cargo file.

```toml
[dependencies]
ink_primitives = { version = "3.0.0-rc6", default-features = false }
ink_metadata = { version = "3.0.0-rc6", default-features = false, features = ["derive"], optional = true }
ink_env = { version = "3.0.0-rc6", default-features = false }
ink_storage = { version = "3.0.0-rc6", default-features = false }
ink_lang = { version = "3.0.0-rc6", default-features = false }
ink_prelude = { version = "3.0.0-rc6", default-features = false }

scale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }
scale-info = { version = "1", default-features = false, features = ["derive"], optional = true }

# These dependencies
pausable = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
psp22 = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
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
   "psp22/std",
   "brush/std",
   "pausable/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract` and import **everything** from `psp22::traits` and `pausable::traits`.

```rust
#[brush::contract]
pub mod my_psp22_pausable {
    use brush::modifiers;
    use pausable::traits::*;
    use psp22::traits::*;
```

## Step 3: Define storage

Declare the storage struct and declare the fields related to the `PausableStorage` and `PSP22Storage` traits. Then you need to derive the `PausableStorage` and `PSP22Storage` traits and mark the corresponding fields with the `#[PausableStorageField]` and `#[PSP22StorageField]` attributes. Deriving these traits allows you to reuse the `PSP22` implementation with a `Pausable` extension.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PausableStorage)]
pub struct MyPSP22Pausable {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PausableStorageField]
    pause: PausableData,
}
```

## Step 4: Inherit logic and implement Pausable logic

Inherit the implementation of the `PSP22` and `Pausable` traits. You can customize (override) methods in this `impl` block. We will implement the `Pausable` logic in this section.

```rust
impl PSP22 for MyPSP22Pausable {
    /// Return `Paused` error if the token is paused
    #[modifiers(when_not_paused)]
    fn _before_token_transfer(
        &mut self,
        _from: &AccountId,
        _to: &AccountId,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        // TODO logic for before token transfer
        Ok(())
    }
}

impl Pausable for MyPSP22Pausable {}
```

## Step 5: Define constructor

Define constructor and add contract functions for pausing and unpausing the contract. Your `PSP22Pausable` contract is ready!

```rust
impl MyPSP22Pausable {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        let mut instance = Self::default();
        assert!(instance._mint(Self::env().caller(), total_supply).is_ok());
        instance
    }

    #[ink(message)]
    pub fn change_state(&mut self) -> Result<(), PSP22Error> {
        if self.paused() {
            self._unpause()
        } else {
            self._pause()
        }
    }
}
```

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).
