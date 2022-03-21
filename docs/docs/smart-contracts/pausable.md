---
sidebar_position: 5
title: Pausable
---

This example shows how you can reuse the implementation of
[pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable) in `Flipper` contract to `flip` only if the contract is not paused.

## Step 1: Include dependencies

Include `brush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of Pausable via `brush` features.

```toml
brush = { tag = "v1.5.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["pausable"] }
```

## Step 2: Add imports and enable unstable feature

Use `brush::contract` macro instead of `ink::contract`. Import **everything** from `brush::contracts::pausable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_pausable {
    use brush::contracts::pausable::*;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to `PausableStorage`.
Then you need to derive `PausableStorage` trait and mark corresponding field
with `#[PausableStorageField]` attribute. Deriving this trait allows you to reuse
the default implementation of `Pausable`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PausableStorage)]
pub struct MyFlipper {
   #[PausableStorageField]
   pause: PausableData,
   flipped: bool,
}
```

## Step 4: Inherit logic

Inherit the implementation of `Pausable`. You can customize (override) methods in this `impl` block.

```rust
impl Pausable for MyFlipper {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `Pausable` contract is ready!

```rust
impl MyFlipper {
   #[ink(constructor)]
   pub fn new() -> Self {
      Self::default()
   }
}
```

## Step 6: Customize your contract

Customize it by adding flipper logic. We will implement `flip` method marked with `when_not_paused` modifier.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_pausable {
    use brush::contracts::pausable::*;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PausableStorage)]
    pub struct MyFlipper {
        #[PausableStorageField]
        pause: PausableData,
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[brush::modifiers(when_not_paused)]
        pub fn flip(&mut self) -> Result<(), PausableError> {
            self.flipped = !self.flipped;
            Ok(())
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
            self._unpause()
        }
    }

    impl Pausable for MyFlipper {}
}
```

You can check an example of the usage of [Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/pausable).
