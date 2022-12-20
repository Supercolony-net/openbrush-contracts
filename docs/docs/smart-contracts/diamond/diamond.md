---
sidebar_position: 1
title: Diamond Standard
---

This example shows how you can use the implementation of [diamond standard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradeability/diamond) to implement diamond standard pattern for upgradeable and unlimited contracts.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `diamond` and `owner` modules, enable corresponding features, and embed modules data structures
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait are `Ownable` and `Diamond`.

## Step 2: Define constructor

Define the constructor and initialize the owner with the contract initiator.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(owner: AccountId, diamond_hash: Hash) -> Self {
        let mut instance = Self::default();
        
        instance._init_with_owner(owner);
        instance.diamond.self_hash = diamond_hash;
        
        instance
    }
}
```

## Step 3: Define forward function

Define the forward function to make delegate calls of facet contracts through the diamond contract.

```rust
impl Contract {
    #[ink(message, payable, selector = _)]
    pub fn forward(&self) {
        self._fallback();
    }
}
```

## Step 4: Customize your contract

You can add more basic functionality for your diamond contract by adding functions to `Contract` implemenation, 
but the point of the Diamond standard is not to increase the size of your contract, 
and to add upgradeable functionality to your contract via so called facets.

When you create a new contract (facet), which you want to make delegate calls from your 
diamond contract to, you will call the `diamond_cut` function on your diamond contract, 
with the code hash of your new facet and the selectors of all the functions from this 
facet you want to use. The diamond will register them and anytime you call this function 
on your diamond contract, it will make the delegate call to the facet the function belongs to. 
You can add, remove or replace these functions anytime with the `diamond_cut` function, 
some of limitations are, that you can not add functions with the same selectors, 
when replacing functions, the new function needs to be from a different contract, 
then currently in use, and when removing functions, the function needs to be registered in the diamond contract.

You can check an example of the usage of [Diamond](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/diamond).
