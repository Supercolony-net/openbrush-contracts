---
sidebar_position: 2
title: PSP35 Batch
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Batch](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/batch.rs) extension, which allows batch transferring of PSP35 tokens.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP35Batch` trait in your `PSP35` smart contract. Add import for `openbrush::contracts::psp35::extensions::batch::*`, inherit the implementation for `PSP35Batch` trait, where you can also customize (override) the original functions from `PSP35Batch`.

```rust
use openbrush::contracts::psp35::extensions::batch::*;

impl PSP35Batch for MyPSP35 {}
```

And that's it! Your `PSP35` is now extended by the `PSP35Batch` extension and ready to use its functions!
You can check an example of the usage of [PSP35 Batch](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/batch).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).
