---
sidebar_position: 2
title: PSP35 Mintable
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/mintable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP35Mintable` trait in your `PSP35` smart contract. Add import for `openbrush::contracts::psp35::extensions::mintable::*`, inherit the implementation for `PSP35Mintable` trait, where you can also customize (override) the original functions from `PSP35Mintable`.

```rust6
use openbrush::contracts::psp35::extensions::mintable::*;

impl PSP35Mintable for MyPSP35 {}
```

And that's it! Your `PSP35` is now extended by the `PSP35Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP35 Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/mintable).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).
