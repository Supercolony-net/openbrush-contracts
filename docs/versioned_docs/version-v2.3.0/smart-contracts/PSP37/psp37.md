---
sidebar_position: 1
title: PSP37
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37) 
token. Also, this example shows how you can customize the logic, for example, to 
track the number of token types with `unique_ids`, adding a new token type with the `add_type` function.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `psp37` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PSP37`.

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

Customize it by adding logic for denying of minting some tokens. 
We can deny minting of token with id by `deny` function.
Id will be added to `denied_ids` map.
If someone tries to mint token with denied id, we will reject transaction.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::traits::String;
    use ink_prelude::{
        vec,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp37::*,
        storage::Mapping,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        denied_ids: Mapping<Id, ()>,
    }

    impl PSP37 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn deny(&mut self, id: Id) {
            self.denied_ids.insert(&id, &());
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP37Error> {
            if self.denied_ids.get(&id).is_some() {
                return Err(PSP37Error::Custom(String::from("Id is denied")))
            }
            self._mint_to(Self::env().caller(), vec![(id, amount)])
        }
    }
}
```
You can check an example of the usage of [PSP37](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp37).

Also you can use extensions for PSP37 token:

[PSP37Metadata](/smart-contracts/PSP37/extensions/metadata): metadata for PSP37.

[PSP37Mintable](/smart-contracts/PSP37/extensions/mintable): creation of new tokens.

[PSP37Burnable](/smart-contracts/PSP37/extensions/burnable): destruction of contract's tokens.

[PSP37Batch](/smart-contracts/PSP37/extensions/batch): transfer batch of tokens.

[PSP37Enumerable](/smart-contracts/PSP37/extensions/enumerable): iterates over contract's tokens.