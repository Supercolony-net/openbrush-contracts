---
sidebar_position: 3
title: PSP35 Burnable
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/burnable.rs) extension.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP35Burnable` trait in your `PSP35` smart contract. Add import for `openbrush::contracts::psp35::extensions::burnable::*`, inherit the implementation for `PSP35Burnable` trait, where you can also customize (override) the original functions from `PSP35Burnable`.

```rust
use openbrush::contracts::psp35::extensions::burnable::*;

impl PSP35Burnable for MyPSP35 {}
```

And that's it! Your `PSP35` is now extended by the `PSP35Burnable` extension and ready to use its functions!
You can check an example of the usage of [PSP35 Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/burnable).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).