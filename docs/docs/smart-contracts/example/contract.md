---
sidebar_position: 8
title: Lending contract
---

Now we will define the contract's logic in our `lib.rs` file.

## Add imports

As everywhere, we will import all we need for our smart contract to work.

```rust
#[brush::contract]
pub mod lending {
    use crate::errors::*;
    use access_control::traits::*;
    use crate::traits::*;
    use ink_prelude::vec::Vec;
    use pausable::traits::*;
    use psp22::{
        extensions::mintable::*,
        traits::*,
    };
```

## Define the events

We will be keeping the track of events that happened in our smart contract. For that, we need to define the event structs and we will be emitting them when needed. We will be emitting the `Lend` event.

```rust
#[ink(event)]
pub struct Lend {
    #[ink(topic)]
    lender: AccountId,
    #[ink(topic)]
    asset: AccountId,
    amount: Balance,
}
```

## Define the contract storage

As described earlier, we want our smart contract to be paused by the Manager accounts. To do that, we need our contract to be `Pausable` and we need a manager role. We can do this with the `AccessControl`. Also, we want to use the `LendingStorageTrait` we have declared. So we will declare a struct and derive all these traits needed.

```rust
#[ink(storage)]
#[derive(Default, AccessControlStorage, PausableStorage, LendingStorage)]
pub struct Lending {
    #[AccessControlStorageField]
    access: AccessControlData,
    #[PausableStorageField]
    pause: PausableData,
    #[LendingStorageField]
    lending: LendingData,
}
```

## Define roles

What about the manager role we have mentioned? We will declare a `const RoleType` Manager and we will allow the admin to grant a Manager role to other accounts.

```rust
const MANAGER: RoleType = ink_lang::selector_id!("MANAGER");
```

## Implement traits

All the traits we need and we have declared storage fields for, we need to implement them so we can call the functions on our smart contract.

```rust
impl LendingStorageTrait for Lending {}

impl AccessControl for Lending {}

impl Pausable for Lending {}
```

## Define the constructor

Finally, we will add a constructor, in which we will initiate the admin of the contract, to whom we will also grant the manager role declared before.

```rust
impl Lending {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_admin(caller);
        instance.grant_role(MANAGER, caller).expect("Can not set manager role");
        instance
    }
}
```

We will take a look at the specific implementation of the functions of the smart contract now.