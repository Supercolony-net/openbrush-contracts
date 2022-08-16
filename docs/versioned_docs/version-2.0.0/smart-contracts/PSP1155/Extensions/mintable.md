---
sidebar_position: 2
title: PSP1155 Mintable
---

This example shows how you can reuse the implementation of [PSP1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp1155) token with [PSP1155Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp1155/src/extensions/mintable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP1155Mintable` trait in your `PSP1155` smart contract. Add import for `openbrush::contracts::psp1155::extensions::mintable::*`, inherit the implementation for `PSP1155Mintable` trait, where you can also customize (override) the original functions from `PSP1155Mintable`.

```rust
use openbrush::contracts::psp1155::extensions::mintable::*;

impl PSP1155Mintable for MyPSP1155 {}
```

And that's it! Your `PSP1155` is now extended by the `PSP1155Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP1155 Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp1155_extensions/mintable).

You can also check the documentation for the basic implementation of [PSP1155](../psp1155.md).
