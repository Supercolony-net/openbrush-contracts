---
sidebar_position: 5
title: PSP22 FlashMint
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22FlashMint](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/src/extensions/flashmint.rs) extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan.

## 1. Implement the FlashMint extension

For your smart contract to use this extension, you need to implement the `PSP22FlashMint` trait in your `PSP22` smart contract. Import everything from `psp22::extensions::flashmint::*` and inherit the implementation for `PSP22FlashMint` trait. You can also customize (override) the original functions from `PSP22FlashMint`.

```rust
use ink_prelude::vec::Vec;
use psp22::{
    extensions::flashmint::*,
    traits::*,
};

impl PSP22FlashMint for MyPSP22FlashMint {}
```

## 2. Implement the FlashLender

You will also need to implement the `FlashLender` trait, where you will call the functions from `PSP22FlashMint`.

```rust
impl FlashLender for MyPSP22FlashMint {
    #[ink(message)]
    fn max_flashloan(&mut self, token: AccountId) -> Balance {
        self._max_flashloan(token)
    }

    #[ink(message)]
    fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
        match self._flash_fee(token, amount) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.into()),
        }
    }

    #[ink(message)]
    fn flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        self._flashloan(receiver_account, token, amount, data)?;
        Ok(())
    }
}
```

And that's it! Your `PSP22` is now extended by the `PSP22FlashMint` extension and ready to use its functions!

You can check the full example of the implementation of this extension [here](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/flashmint).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).