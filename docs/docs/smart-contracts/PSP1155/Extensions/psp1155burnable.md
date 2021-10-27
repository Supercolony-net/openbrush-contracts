---
sidebar_position: 1
title: PSP1155 Burnable
---

This example shows how you can reuse the implementation of [PSP1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155) token with [PSP1155Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155/src/extensions/burnable.rs) extension.

## How to use this extension

In order for your smart contract to use this extension, you only need to implement the `PSP1155Burnable` trait in your `PSP1155` smart contract. Add import for  `psp1155::extensions::burnable::*`, inherit the implementation for `PSP1155Burnable` trait, where you can also customize (override) the original functions from `PSP1155Burnable`.

```rust
use psp1155::extensions::burnable::*;

impl PSP1155Burnable for MyPSP1155 {}
```

And that's it! Your `PSP1155` is now extended by the `PSP1155Burnable` extension and ready to use it's functions!
