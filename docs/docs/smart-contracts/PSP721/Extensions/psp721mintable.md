---
sidebar_position: 2
title: PSP721 Mintable
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/mintable.rs) extension.

## Step 1: Include dependencies

Include dependencies on `psp721` and `brush` in the cargo file. We will include the same dependencies as in [PSP721](/smart-contracts/PSP721/psp721).

## Step 2: Add imports

Similarly as in [PSP721](/smart-contracts/PSP721/psp721) imports, replace `ink::contract` macro by `brush::contract`, and import **everything** from `psp721::traits`. You will also need to import `psp721::extensions::mintable`.

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

Declare storage struct the same way as in [PSP721](/smart-contracts/PSP721/psp721).

## Step 4: Inherit logic

Inherit implementations of `PSP721` and `PSP721Mintable` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP721 for MyPSP721 {}

impl PSP721Mintable for MyPSP721 {}
```

## Step 5: Define constructor

Define constructor for your smart  contract the same way as in [PSP721](/smart-contracts/PSP721/psp721). Your basic version of `PSP721Burnable` contract is ready!
