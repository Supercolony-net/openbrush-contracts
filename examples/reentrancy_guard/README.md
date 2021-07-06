## Overview
This example shows how you can use the [non_reentrant](contracts/security/reentrancy_guard)
modifier to prevent reentrancy into certain functions.
In this example we will create two contract:
- `my_flipper_guard` - is contract of simple [flipper](https://github.com/paritytech/ink/tree/master/examples/flipper)
  but method `flip` will be marked with `non_reentrant` modifier +
  we will add additional method, also marked with `non_reentrant`, 
  which will ask another contract to call `flip` of our `flipper`.
- `flip_on_me` - is contract which has only one method `flip_on_me`.
  This method will try to call `flip` on caller
  (it means that caller must be a contract with method `flip`).

## MyFlipper

### Steps
1. You need to include `reentrancy_guard` and `brush` in cargo file.
```markdown
[dependencies]
...

reentrancy_guard = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "brush/std",
]
```
2. To declare the contract, you need to use `brush::contract` macro instead of `ink::contract`.
   Import **everything** from `reentrancy_guard` trait module.
```rust
#[brush::contract]
pub mod my_flipper_guard {
    use reentrancy_guard::traits::*;
    use brush::modifiers;
    use ink_env::call::FromAccountId;
    use crate::flip_on_me::CallerOfFlip;
```
3. Declare storage struct and derive `ReentrancyGuardStorage`trait. Deriving this trait 
   will add required field to your structure for modifier.
```rust
#[ink(storage)]
#[derive(Default, ReentrancyGuardStorage)]
pub struct MyFlipper {
    value: bool,
}
```
4. After that you can add `reentrancy_guard` modifier to `flip` and `call_flip_on_me` methods.
```rust
impl MyFlipper {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[ink(message)]
    pub fn get_value(&self) -> bool {
        self.value
    }

    #[ink(message)]
    #[brush::modifiers(non_reentrant)]
    pub fn flip(&mut self) {
        self.value = !self.value;
    }

    #[ink(message)]
    #[modifiers(non_reentrant)]
    pub fn call_flip_on_me(&mut self, callee: AccountId) {
        // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.
        // Callee contract during execution of `flip_on_me` will call `flip` of this contract.
        // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,
        // that call of `flip` after `call_flip_on_me` must fail.
        let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);
        flipper.flip_on_me();
    }
}
```
5. To simplify cross contract call to `FlipOnMe` contract let's create wrapper around account id of contract. 
   For that we will define another contract in this crate with `#[ink_lang::contract(compile_as_dependency = true)]`
   and empty methods but with the same signature as in original contract.
```rust
/// It is stub implementation of contract with method `flip_on_me`.
/// We need this implementation to create wrapper around account id of contract.
/// With this wrapper we easy can call method of some contract.
/// Example:
/// ```
/// let mut flipper: CallerOfFlip = FromAccountId::from_account_id(calle);
/// flipper.flip_on_me();
/// ```
#[ink_lang::contract(compile_as_dependency = true)]
pub mod flip_on_me {
    #[ink(storage)]
    pub struct CallerOfFlip {}

    impl CallerOfFlip {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    impl CallerOfFlip {
        #[ink(message)]
        pub fn flip_on_me(&mut self) {
            unimplemented!()
        }
    }
}
```

## FlipOnMe

It is a simple contract which doesn't use any logic from the brush, so you can use simple ink! here.

### Steps
1. Define `FlipOnMe` contract. It has only method `flip_on_me` which will call `flip` on caller.
```rust
#[ink_lang::contract]
pub mod flip_on_me {
    use ink_env::call::FromAccountId;
    use crate::flipper::Flipper;

    #[ink(storage)]
    #[derive(Default)]
    pub struct FlipOnMe {}

    impl FlipOnMe {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn flip_on_me(&mut self) {
            let caller = self.env().caller();
            // This method will do a cross-contract call to caller account. It will try to call `flip`
            let mut flipper: Flipper = FromAccountId::from_account_id(caller);
            flipper.flip();
        }
    }
}
```
2. To simplify cross contract call to `Flipper` contract let's create wrapper around account id of contract.
   For that we will define another contract in this crate with `#[ink_lang::contract(compile_as_dependency = true)]`
   and empty methods but with the same signature as in original contract.
```rust
/// It is stub implementation of contract with method `flip`.
/// We need this implementation to create wrapper around account id of contract.
/// With this wrapper we easy can call method of some contract.
/// Example:
/// ```
/// let mut flipper: Flipper = FromAccountId::from_account_id(caller);
/// flipper.flip();
/// ```
#[ink_lang::contract(compile_as_dependency = true)]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {}

    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    impl Flipper {
        #[ink(message)]
        pub fn flip(&mut self) {
            unimplemented!()
        }
    }
}
```
## Testing
For testing, you can run according [integration test](tests/reentrancy_guard.tests.ts).
Or you need to deploy both contracts and call `call_flip_on_me` on `MyFlipper` 
account and pass the account id of `FlipOnMe` contract.
