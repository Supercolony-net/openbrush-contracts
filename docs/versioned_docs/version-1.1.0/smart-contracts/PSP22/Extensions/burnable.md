---
sidebar_position: 3
title: PSP22 Burnable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/burnable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP22Burnable` trait in your `PSP22` smart contract. Add import for  `psp22::extensions::burnable::*`, inherit the implementation for `PSP22Burnable` trait, where you can also customize (override) the original functions from `PSP22Burnable`.

```rust
use psp22::extensions::burnable::*;

impl PSP22Burnable for MyPSP22 {}
```

And that's it! Your `PSP22` is now extended by the `PSP22Burnable` extension and ready to use its functions!

You can also check the documentation for the basic implementation of [PSP22](../psp22.md).