---
sidebar_position: 2
title: PSP22 Mintable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) token with [PSP22Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/mintable.rs) extension via `pallet-assets` chain extension.

## How to use this extension

First, you should implement basic version of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).

For your smart contract to use this extension, you only need to implement the
`PSP22Mintable` trait in your `PSP22 Pallet` smart contract. Add import for
`openbrush::contracts::psp22_pallet::extensions::mintable::*`, inherit the implementation for
`PSP22Mintable` trait, where you can also customize (override) the original functions
from `PSP22Mintable`.

```rust
use openbrush::contracts::psp22_pallet::extensions::mintable::*;

impl PSP22Mintable for Contract {}
```

[//]: # (You can check an example of the usage of [PSP22 Mintable]&#40;https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_pallet_extensions/mintable&#41;.)

And that's it! Your `PSP22 Pallet` is now extended by the `PSP22Mintable` extension and ready to use its functions!
