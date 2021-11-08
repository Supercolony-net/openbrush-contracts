---
sidebar_position: 3
title: PSP22Burnable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/burnable.rs) extension.

## Step 1: Add import

Add import of PSP22Burnable extension to your contract.

```rust
use psp22::extensions::burnable::*;
```

## Step 2: Inherit logic

Inherit implementations of `PSP22Burnable` trait. You can customize (override) methods in this `impl` block.

Your basic version of `PSP22` contract with 'PSP22Burnable extension is ready!

```rust
impl PSP22Burnable for MyPSP22 {}
```


## Step 3: Customize your contract

Customize it by adding functionality of burning from many accounts.

```rust
impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance) -> Self {
        let mut instance = Self::default();
        instance._mint(instance.env().caller(), _total_supply);
        instance
    }

    #[ink(message)]
    pub fn burn_from_many(&mut self, accounts: Vec<(AccountId, Balance)>) {
        for account in accounts.iter() {
            self.burn_from(account.0, account.1);
        }
    }
}
```

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).