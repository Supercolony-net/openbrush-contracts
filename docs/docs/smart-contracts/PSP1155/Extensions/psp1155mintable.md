---
sidebar_position: 2
title: PSP1155 Mintable
---

This example shows how you can reuse the implementation of [PSP1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155) token with [PSP1155Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155/src/extensions/mintable.rs) extension.

## Step 1: Include dependencies

Include dependencies on `psp1155` and `brush` in the cargo file. We will include the same dependencies as in [PSP1155](/smart-contracts/PSP1155/psp1155).

## Step 2: Add imports

Similarly as in [PSP1155](/smart-contracts/PSP1155/psp1155) imports, replace `ink::contract` macro by `brush::contract`, and import **everything** from `psp1155::traits`. You will also need to import `psp1155::extensions::mintable`.

## Step 3: Define storage

Declare storage struct the same way as in [PSP1155](/smart-contracts/PSP1155/psp1155).

## Step 4: Inherit logic

Inherit implementations of `PSP1155` and `PSP1155Mintable` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP1155 for MyPSP1155 {}

impl PSP1155Mintable for MyPSP1155 {}
```

## Step 5: Define constructor

Define constructor for your smart contract the same way as in [PSP1155](/smart-contracts/PSP1155/psp1155). Your basic version of `PSP1155Burnable` contract is ready!
