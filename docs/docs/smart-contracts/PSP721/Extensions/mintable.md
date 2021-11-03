---
sidebar_position: 2
title: PSP721 Mintable
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/mintable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP721Mintable` trait in your `PSP721` smart contract. Add import for  `psp721::extensions::mintable::*`, inherit the implementation for `PSP721Mintable` trait, where you can also customize (override) the original functions from `PSP721Mintable`.

```rust
use psp721::extensions::mintable::*;

impl PSP721Mintable for MyPSP721 {}
```

And that's it! Your `PSP721` is now extended by the `PSP721Mintable` extension and ready to use its functions!

You can also check the documentation for the basic implementation of [PSP721](/smart-contracts/PSP721/psp721).
