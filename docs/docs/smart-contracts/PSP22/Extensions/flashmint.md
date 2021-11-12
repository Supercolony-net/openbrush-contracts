---
sidebar_position: 5
title: PSP22 FlashMint
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22FlashMint](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/flashmint.rs) extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan.

## How to use this extension

For your smart contract to use this extension, you only need to implement the `PSP22FlashMint` trait in your `PSP22` smart contract. Add import for  `psp22::extensions::flashmint::*`, inherit the implementation for `PSP22FlashMint` trait, where you can also customize (override) the original functions from `PSP22FlashMint`.

```rust
use psp22::extensions::flashmint::*;

impl PSP22FlashMint for MyPSP22 {}
```

And that's it! Your `PSP22` is now extended by the `PSP22FlashMint` extension and ready to use its functions!

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).