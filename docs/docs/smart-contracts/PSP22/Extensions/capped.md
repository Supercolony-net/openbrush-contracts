---
sidebar_position: 7
title: PSP22 Capped
---

This example shows how you can implement a [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) contract with a [PSP22Capped](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/extension/capped) extension.

## Step 1: Add imports

Replace `ink::contract` macro by `brush::contract` and import **everything** from `psp22::traits` and `psp22::extensions::capped`.

```rust
#[brush::contract]
pub mod my_psp22_pausable {
    use psp22::traits::*;
    use psp22::extensions::capped::*;
```

## Step 2: Define storage

Declare the storage struct and declare the fields related to the `PSP22CappedStorage` and `PSP22Storage` traits. Then you need to derive the `PSP22CappedStorage` and `PSP22Storage` traits and mark the corresponding fields with the `#[PSP22CappedStorageField]` and `#[PSP22StorageField]` attributes. Deriving these traits allows you to reuse the `PSP22` implementation with a `PSP22Capped` extension.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PSP22CappedStorage)]
pub struct MyPSP22Capped {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22CappedStorageField]
    capped: PSP22CappedData,
}
```

## Step 3: Inherit logic

Inherit the implementation of the `PSP22` and `PSP22Capped` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP22 for MyPSP22Capped {}

impl PSP22Capped for MyPSP22Capped {}
```

## Step 4: Define constructor and override mint logic

Define constructor and override the mint function to check for cap overflow. Your `PSP22Capped` contract is ready!

```rust
impl MyPSP22Capped {
    #[ink(constructor)]
    pub fn new(inital_supply: Balance, cap: Balance) -> Self {
        let mut instance = Self::default();
        assert!(instance.init_cap(cap).is_ok());
        assert!(instance._mint(instance.env().caller(), inital_supply).is_ok());
        instance
    }

    /// Add cap overflow check and then call the original `_mint` function
    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if (self.total_supply() + amount) > self.cap() {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        PSP22::_mint(self, account, amount)
    }
}
```

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).
