---
sidebar_position: 1
title: PSP34
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) token. Also, this example shows how you can customize the logic, for example, to track the number of tokens minted with `next_id`, increasing it with each new token minted, securing a unique id for each token.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `psp34` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PSP34`.

## Step 2: Define constructor

Define empty constructor.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
    }
}
```

## Step 3: Customize your contract

Customize it by adding logic for tracking the number of minted tokens. 
It will contain a custom `mint_token` function which will handle the id of the 
newly minted token. Also, we will add the `next_id: u8` field to the structure, 
which will be increased with each newly minted token. This way we will make sure 
there will always be added a token with a unique id. 

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use ink::storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
        next_id: u8,
    }

    impl PSP34 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), id)
        }
    }
}
```

You can check an example of the usage of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34).
Also you can use extensions for psp34 token:

[PSP34Metadata](/smart-contracts/PSP34/extensions/metadata): metadata for PSP34.

[PSP34Mintable](/smart-contracts/PSP34/extensions/mintable): creation of new tokens.

[PSP34Burnable](/smart-contracts/PSP34/extensions/burnable): destruction of contract's tokens.

[PSP34Enumerable](/smart-contracts/PSP34/extensions/enumerable): iterating over contract's tokens.
