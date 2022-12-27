---
sidebar_position: 2
title: PSP37 Batch
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Batch](https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/token/psp37/extensions/batch.rs) extension, which allows batch transferring of PSP37 tokens.

## How to use this extension

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

For your smart contract to use this extension, you only need to implement the `PSP37Batch` trait in your 
`PSP37` smart contract. Add import for `openbrush::contracts::psp37::extensions::batch::*`, 
inherit the implementation for `PSP37Batch` trait, where you can also customize (override) 
the original functions from `PSP37Batch`.

```rust
use openbrush::contracts::psp37::extensions::batch::*;

impl PSP37Batch for Contract {}
```

And that's it! Your `PSP37` is now extended by the `PSP37Batch` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Batch](https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp37_extensions/batch).
