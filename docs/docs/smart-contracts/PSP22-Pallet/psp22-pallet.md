---
sidebar_position: 1
title: PSP22 Pallet
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) via `pallet-assets` chain extension. Also, this example shows how you can customize the logic, for example, to reject transferring tokens to `hated_account`.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `psp22_pallet` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PSP22`.

## Step 2: Define storage and implement default PSP22 trait

Use `psp22_pallet` storage and implement `PSP22` trait for your contract.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22_pallet::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}
}
```

## Step 3: Add constructor

Add constructor for your contract, create asset and mint tokens to caller.

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
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP22 for Contract {}

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {
                // The contract is admin of the asset
                instance
                    ._create(asset_id, Self::env().account_id(), min_balance)
                    .expect("Should create an asset");
                instance.pallet.asset_id = asset_id;
                instance.pallet.origin = Origin::Caller;
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }
    }
}
```

You can check an example of the usage of [PSP22 Pallet](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_pallet).

Also you can use extensions for PSP22 token:

[PSP22 Pallet Metadata](/smart-contracts/psp22_pallet/extensions/metadata): metadata for PSP22 Pallet.

[PSP22 Pallet Mintable](/smart-contracts/psp22_pallet/extensions/mintable): creation of new tokens.

[PSP22 Pallet Burnable](/smart-contracts/psp22_pallet/extensions/burnable): destruction of own tokens.
