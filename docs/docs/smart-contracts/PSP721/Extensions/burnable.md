---
sidebar_position: 3
title: PSP721 Burnable
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/burnable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP721Burnable` trait in your `PSP721` smart contract. Add import for  `psp721::extensions::burnable::*`, inherit the implementation for `PSP721Burnable` trait, where you can also customize (override) the original functions from `PSP721Burnable`.

```rust
use psp721::extensions::burnable::*;

impl PSP721Burnable for MyPSP721 {}
```

And that's it! Your `PSP721` is now extended by the `PSP721Burnable` extension and ready to use its functions!

You can also check the documentation for the basic implementation of [PSP721](/smart-contracts/PSP721/psp721).
