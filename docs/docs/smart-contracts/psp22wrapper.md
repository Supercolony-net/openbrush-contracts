---
sidebar_position: 8
title: PSP22 Wrapper
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with [PSP22 Wrapper](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/extensions/wrapper) extension.

## Step 1: Include dependencies and add imports

Include dependencies to `psp22` and `brush` in the cargo file. Then we need to replace `ink::contract` macro by `brush::contract` and import **everything** from `psp22::traits` and `psp22::extensions::wrapper`.

```rust
#[brush::contract]
pub mod my_psp22 {
   use psp22::{
        extensions::wrapper::*,
        traits::*,
    };
```

## Step 2: Define storage

Declare storage struct and declare the fields related to `PSP22Storage` and `PSP22WrapperStorage` traits. Then you need to derive `PSP22Storage` and `PSP22WrapperStorage` traits and mark corresponding fields with `#[PSP22StorageField]` and `#[PSP22WrapperStorageField]` attributes. Deriving these traits allows you to reuse
the default implementation of `PSP22` and `PSP22Wrapper`.

```rust
#[ink(storage)]
#[derive(Default, PSP22WrapperStorage, PSP22Storage)]
pub struct MyPSP22Wrapper {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22WrapperStorageField]
    wrapper: PSP22WrapperData,
}
```

## Step 3: Inherit logic

Inherit implementations of `PSP22`, `PSP22Wrapper` and `PSP22Receiver` traits. We need to implement the `PSP22Receiver` in order for our smart contract to be able to recieve the underlying `PSP22` tokens. Your smart contract may panic with `TrapCode::UnreachableCodeReached` when transfering the underlying token, if you do not implement the `PSP22Receiver` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP22 for MyPSP22Wrapper {}

impl PSP22Wrapper for MyPSP22Wrapper {}

impl PSP22Receiver for MyPSP22Wrapper {
        #[ink(message)]
        fn before_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22ReceiverError> {
            // TODO your logic for before_recieved function
            Ok(())
        }
    }
```

## Step 4: Define constructor

Define constructor. Your implementation of `PSP22Wrapper` contract is ready!

```rust
impl MyPSP22 {
   #[ink(constructor)]
   pub fn new(token_address: AccountId) -> Self {
        let mut instance = Self::default();
        instance.init(token_address);
        instance
    }
}
```
