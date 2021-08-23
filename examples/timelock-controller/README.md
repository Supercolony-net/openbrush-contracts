## Overview

This example shows how you can reuse the implementation of
[timelock-controller](contracts/governance/timelock-controller).

## Steps

1. You need to include `timelock-controller` and `brush` in cargo file.

```markdown
[dependencies]
...

timelock-controller = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "timelock-controller/std",
   "brush/std",
]
```

2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`. Import **everything**
   from corresponding trait modules.

```rust
#[brush::contract]
pub mod my_timelock_controller {
   use timelock_controller::traits::*;
   use ink_prelude::vec::Vec;
```

3. `TimelockController` is an extension for `AccessControl`, so you have to implement logic related to both modules.
   Declare storage struct and declare the fields related to `TimelockControllerStorage` and `AccessControlStorage`.
   Then you need to derive `TimelockControllerStorage` and `AccessControlStorage` traits and mark corresponsing fields
   with `#[TimelockControllerStorageField]` and `#[AccessControlStorageField]` attributes. 
   Deriving these traits allows you to reuse the default implementation of `TimelockController`(and `AccessControl`).

```rust
#[ink(storage)]
#[derive(Default, AccessControlStorage, TimelockControllerStorage)]
pub struct TimelockStruct {
   #[AccessControlStorageField]
   access: AccessControlData,
   #[TimelockControllerStorageField]
   timelock: TimelockControllerData,
}
```

4. After that you can inherit the implementation of `TimelockController` and `AccessControl` traits. You can customize (override) some
   methods there.

```rust
// `TimelockController` is an extension for `AccessControl`, so you have to implement logic related to both modules.
impl AccessControl for TimelockStruct {}
impl TimelockController for TimelockStruct {}
```

5. Now you only need to define constructor and your basic version of `TimelockController` contract is ready.

```rust
impl TimelockStruct {
   #[ink(constructor)]
   pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
      let mut instance = Self::default();
      let caller = instance.env().caller();
      // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
      // You need to call it for each trait separately, to initialize everything for these traits.
      AccessControl::_init_with_admin(&mut instance, caller);
      TimelockController::_init_with_admin(&mut instance, caller, min_delay, proposers, executors);
      instance
   }
}
```
