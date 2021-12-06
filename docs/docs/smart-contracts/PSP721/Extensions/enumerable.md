---
sidebar_position: 4
title: PSP721 Enumerable
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/enumerable.rs) extension. This extension allows you to enumerate over contract's NFTs, as well as over user's NFTs.

## Step 1: Add imports

Import **everything** from `psp721::extensions::enumerable` and `psp721::traits`.

```rust
#[brush::contract]
pub mod my_psp721 {
   use psp721::{
        extensions::enumerable::*,
        traits::*,
    };
```

## Step 2: Define storage

Declare storage struct and declare the field related to the `PSP721EnumerableStorage` trait in addition to your `PSP721Storage` field. Then you need to derive the `PSP721EnumerableStorage` trait and mark the corresponding field with the `#[PSP721EnumerableStorageField]` attribute. Deriving this trait allows you to reuse the `PSP721Enumerable` extension in your `PSP721` implementation.

```rust
#[derive(Default, PSP721Storage, PSP721EnumerableStorage)]
#[ink(storage)]
pub struct MyPSP721 {
    #[PSP721StorageField]
    psp721: PSP721Data,
    #[PSP721EnumerableStorageField]
    enumerable: PSP721EnumerableData,
}
```

## Step 3: Inherit logic and override functions

Inherit implementation of the `PSP721Enumerable` trait and override the `_before_token_transfer` function to call function for handling the enumerable data. You can customize (override) other functions in this `impl` block.

```rust
impl PSP721 for MyPSP721 {
    fn _before_token_transfer(&mut self, from: &AccountId, to: &AccountId, id: &Id) -> Result<(), PSP721Error> {
        self._handle_token_transfer(from, to, id);
        Ok(())
    }
}

impl PSP721Enumerable for MyPSP721 {}
```

And that's it! You now extended your `PSP721` contract by enumerable extension!

You can also check the documentation for the basic implementation of [PSP721](/smart-contracts/PSP721/psp721).
