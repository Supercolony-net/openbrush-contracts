---
sidebar_position: 7
title: Payment Splitter
---

This example shows how you can reuse the implementation of
[payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/finance/payment_splitter).

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `payment_splitter` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PaymentSplitter`.

## Step 2: Define constructor

Define constructor where you init payees and shares.

```rust
impl Contract {
   #[ink(constructor)]
   pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
       
      let mut instance = Self::default();
      instance._init(payees_and_shares).expect("Should init");
      
      instance 
   }
}
```

You can check an example of the usage of [PaymentSplitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/payment_splitter).

## Step 3 (Optional): Customize your contract

The `PaymentSplitter` trait defines and has default implementations for the core payment splitter functions.
Additional functionality with *some* predefined functions is available through the `payment_splitter::Internal` trait. 
Likely the most common function to use from this internal trait will be `_release_all`. This allows you to payout all 
`payees` stored in the contract at once. To add this function to your contract, simply define a new publicly dispatchable 
function (i.e. `#[ink(message)]`) called `release_all` and have it call the internal `_release_all` function using `self`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_payment_splitter {
    use ink::prelude::vec::Vec;
    use openbrush::contracts::payment_splitter::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        splitter: payment_splitter::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            let mut instance = Self::default();
            
            instance._init(payees_and_shares).expect("Should init");
            
            instance
        }

        /// Payout all payees at once.
        /// Delete this method if you don't want this functionality in your version of the payment splitter.
        #[ink(message)]
        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {
            // `_release_all()` is an internal method defined by the `payment_splitter::Internal` trait
            self._release_all()
        }
    }

    impl PaymentSplitter for Contract {}
}
```
The `_add_payee` function is also available in the `payment_splitter::Internal` trait and can be added to 
your contract in the same way as `_release_all`.