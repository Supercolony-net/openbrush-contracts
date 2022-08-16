---
sidebar_position: 2
title: PSP22 Mintable
---

This example shows how you can reuse the implementation of
[PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/mintable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP22Mintable` trait in your `PSP22` smart contract. Add import for `openbrush::contracts::psp22::extensions::mintable::*`, inherit the implementation for `PSP22Mintable` trait, where you can also customize (override) the original functions from `PSP22Mintable`.

```rust
use openbrush::contracts::psp22::extensions::mintable::*;

impl PSP22Mintable for MyPSP22 {}
```

You can check an example of the usage of [PSP22 Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/mintable).

And that's it! Your `PSP22` is now extended by the `PSP22Mintable` extension and ready to use its functions!

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).