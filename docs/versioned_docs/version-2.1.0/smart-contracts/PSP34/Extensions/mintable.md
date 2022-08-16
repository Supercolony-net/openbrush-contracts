---
sidebar_position: 2
title: PSP34 Mintable
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/mintable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP34Mintable` trait in your `PSP34` smart contract. Add import for `openbrush::contracts::psp34::extensions::mintable::*`, inherit the implementation for `PSP34Mintable` trait, where you can also customize (override) the original functions from `PSP34Mintable`.

```rust
use openbrush::contracts::psp34::extensions::mintable::*;

impl PSP34Mintable for MyPSP34 {}
```

And that's it! Your `PSP34` is now extended by the `PSP34Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP34 Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34_extensions/mintable).

You can also check the documentation for the basic implementation of [PSP34](/smart-contracts/PSP34).
