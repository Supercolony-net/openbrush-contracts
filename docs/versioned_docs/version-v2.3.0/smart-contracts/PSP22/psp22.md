---
sidebar_position: 1
title: PSP22
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) token. Also, this example shows how you can customize the logic, for example, to reject transferring tokens to `hated_account`.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `psp22` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PSP22`.

## Step 2: Define constructor

Define constructor where you mint tokens to caller.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance
                ._mint_to(instance.env().caller(), total_supply)
                .expect("Should mint");
        })
    }
}
```

## Step 3: Customize your contract

Customize it by adding hated account logic. It will contain two public methods `set_hated_account` and `get_hated_account`. 
Also we will override `_before_token_transfer` method in the `PSP22` implementation(that methods defined in `Transfer` trait), 
and we will add the `hated_account: AccountId` field to the structure.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::traits::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl Transfer for Contract {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to == Some(&self.hated_account) {
                return Err(PSP22Error::Custom(String::from("I hate this account!")));
            }
            Ok(())
        }
    }

    impl PSP22 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {
                instance
                    ._mint_to(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }
    }
}
```

You can check an example of the usage of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22).

Also you can use extensions for PSP22 token:

[PSP22Metadata](/smart-contracts/psp22/extensions/metadata): metadata for PSP22.

[PSP22Mintable](/smart-contracts/psp22/extensions/mintable): creation of new tokens.

[PSP22Burnable](/smart-contracts/psp22/extensions/burnable): destruction of own tokens.

[PSP22Wrapper](/smart-contracts/psp22/extensions/wrapper): token wrapper for PSP22.

[PSP22FlashMint](/smart-contracts/psp22/extensions/flashmint): extension which allows the user to perform flashloans on the token by minting and burning the token.

Check out the utilities for PSP22 token:

[PSP22TokenTimelock](/smart-contracts/psp22/utils/token-timelock): utility for locking PSP22 tokens for a specified time.
