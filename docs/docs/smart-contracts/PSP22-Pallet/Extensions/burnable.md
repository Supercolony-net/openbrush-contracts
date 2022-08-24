---
sidebar_position: 3
title: PSP22 Burnable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) token with [PSP22Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/burnable.rs) extension via `pallet-assets` chain extension.

## How to use this extension

First, you should implement basic version of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).

For your smart contract to use this extension, you only need to implement the
`PSP22Burnable` trait in your `PSP22 Pallet` smart contract. Add import for
`openbrush::contracts::psp22_pallet::extensions::burnable::*`, inherit the
implementation for `PSP22Burnable` trait, where you can also customize (override)
the original functions from `PSP22Burnable`.

```rust
use openbrush::contracts::psp22_pallet::extensions::burnable::*;

impl PSP22Burnable for Contract {}
```

And that's it! Your `PSP22 Pallet` is now extended by the `PSP22Burnable` extension and ready to use its functions!

[//]: # (You can check an example of the usage of [PSP22 Burnable]&#40;https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_pallet_extensions/burnable&#41;.)
