---
sidebar_position: 2
title: PSP22Mintable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/mintable.rs) extension.

## Step 1: Add import

Add import of PSP22Mintable extension to your contract.

```rust
use psp22::extensions::mintable::*;
```

## Step 2: Inherit logic

Inherit implementations and `PSP22Mintable` trait. You can customize (override) methods in this `impl` block.

Your basic version of `PSP22` contract with 'PSP22Mintable' extension is ready!

```rust
impl PSP22Mintable for MyPSP22 {}
```

## Step 6: Customize your contract

Customize it by adding functionality of minting to account.

```rust

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

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).