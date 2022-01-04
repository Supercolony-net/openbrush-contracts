---
sidebar_position: 3
title: PSP1155 Burnable
---

This example shows how you can reuse the implementation of [PSP1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155) token with [PSP1155Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155/src/extensions/burnable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP1155Burnable` trait in your `PSP1155` smart contract. Add import for `brush::contracts::psp1155::extensions::burnable::*`, inherit the implementation for `PSP1155Burnable` trait, where you can also customize (override) the original functions from `PSP1155Burnable`.

```rust
use brush::contracts::psp1155::extensions::burnable::*;

impl PSP1155Burnable for MyPSP1155 {}
```

And that's it! Your `PSP1155` is now extended by the `PSP1155Burnable` extension and ready to use its functions!
You can check an example of the usage of [PSP1155 Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp1155_extensions/burnable).

You can also check the documentation for the basic implementation of [PSP1155](/smart-contracts/PSP1155/psp1155).