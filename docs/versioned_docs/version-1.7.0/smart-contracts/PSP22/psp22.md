---
sidebar_position: 1
title: PSP22
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token. Also, this example shows how you can customize the logic, for example, to reject transferring tokens to `hated_account`.

## Step 1: Include dependencies

Include `brush` as dependency in the cargo file or you can use [default `Cargo.toml`](../overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP22 via `brush` features.

```toml
brush = { tag = "v1.7.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["psp22"] }
```

## Step 2: Add imports and enable unstable feature

Use `brush::contract` macro instead of `ink::contract`. Import **everything** from `brush::contracts::psp22`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22 {
    use brush::contracts::psp22::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 3: Define storage

Declare the storage struct and declare the field related to the `PSP22Storage` trait. Then you need to derive the `PSP22Storage` trait and mark the corresponding field with the `#[PSP22StorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `PSP22`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
}
```

## Step 4: Inherit logic

Inherit the implementation of `PSP22` trait. You can customize (override) methods in the `impl` block.

```rust
impl PSP22 for MyPSP22 {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `PSP22` contract is ready!

```rust
impl MyPSP22 {
   #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut MyPSP22| {
            instance
                ._mint(instance.env().caller(), total_supply)
                .expect("Should mint");
        })
    }
}
```

## Step 6: Customize your contract

Customize it by adding hated account logic. It will contain two public methods `set_hated_account` and `get_hated_account`. Also we will
override `_before_token_transfer` method in the `PSP22` implementation(that methods defined in `PSP22Transfer` trait), and we will add the `hated_account: AccountId` field to the structure.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
    // fields for hater logic
    hated_account: AccountId,
}

impl PSP22Transfer for MyPSP22 {
    // Let's override method to reject transactions to bad account
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if to == Some(&self.hated_account) {
            return Err(PSP22Error::Custom(String::from("I hate this account!")))
        }
        Ok(())
    }
}

impl PSP22 for MyPSP22 {}

impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut MyPSP22| {
            instance
                ._mint(instance.env().caller(), total_supply)
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
```

You can check an example of the usage of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22).

Also you can use extensions for PSP22 token:

[PSP22Metadata](Extensions/metadata): metadata for PSP22.

[PSP22Mintable](Extensions/mintable): creation of new tokens.

[PSP22Burnable](Extensions/burnable): destruction of own tokens.

[PSP22Wrapper](Extensions/wrapper): token wrapper for PSP22.

[PSP22FlashMint](Extensions/flashmint): extension which allows the user to perform flashloans on the token by minting and burning the token.

[PSP22Capped](Extensions/capped): extension which adds a cap for total supply of PSP22 tokens.

Check out the utilities for PSP22 token:

[PSP22TokenTimelock](Utils/token-timelock): utility for locking PSP22 tokens for a specified time.
