---
sidebar_position: 5
title: PSP22 FlashMint
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22FlashMint](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/flashmint.rs) extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan.

## 1. Implement the FlashMint extension

For your smart contract to use this extension, you need to implement the `PSP22FlashMint` trait in your `PSP22` smart contract. Import everything from `openbrush::contracts::psp22::extensions::flashmint::*` and inherit the implementation for `PSP22FlashMint` trait. You can also customize (override) the original functions from `PSP22FlashMint`.

```rust
use openbrush::contracts::psp22::extensions::flashmint::*;

impl FlashLender for MyPSP22FlashMint {}
```

And that's it! Your `PSP22` is now extended by the `PSP22FlashMint` extension and ready to use its functions!
You can check the full example of the implementation of this extension [here](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/flashmint).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).