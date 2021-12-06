---
sidebar_position: 7
title: PSP22 Capped
---

This example shows how you can implement a [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) contract with a supply cap, analogue to [ERC20Capped](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/extensions/ERC20Capped.sol).

## Step 1: Define storage

Declare the storage struct and the field related to the `PSP22Storage` trait, derive the `PSP22Storage` trait and mark the corresponding field with the `#[PSP22StorageField]` attribute. Also add the storage variable for cap.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage)]
pub struct MyPSP22Capped {
    #[PSP22StorageField]
    psp22: PSP22Data,
    cap: Balance,
}
```

## Step 2: Define constructor and contract functions

Define constructor, inherit `PSP22`, and override the basic functions for capped implementation. Your `PSP22Capped` contract is ready!

```rust
impl PSP22 for MyPSP22Capped {}

impl MyPSP22Capped {
    #[ink(constructor)]
    pub fn new(inital_supply: Balance, cap: Balance) -> Self {
        let mut instance = Self::default();
        assert!(instance.init_cap(cap).is_ok());
        assert!(instance._mint(instance.env().caller(), inital_supply).is_ok());
        instance
    }

    /// Expose the `_mint` function
    #[ink(message)]
    pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint(account, amount)
    }

    #[ink(message)]
    /// Method to return token's cap
    pub fn cap(&self) -> Balance {
        self.cap
    }

    /// Overrides the `_mint` function to check for cap overflow before minting tokens
    /// Performs `PSP22::_mint` after the check succeeds
    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if (self.total_supply() + amount) > self.cap() {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        PSP22::_mint(self, account, amount)
    }

    /// Initializes the token's cap
    fn init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
        if cap <= 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
        }
        self.cap = cap;
        Ok(())
    }
}
```

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).
