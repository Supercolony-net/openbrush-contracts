---
sidebar_position: 5
title: Pausable
---

This example shows how you can reuse the implementation of
[pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable) in `Flipper` contract to `flip` only if the contract is not paused.

## Step 1: Include dependencies

Include dependencies to `pausable` and `brush` in the cargo file.

```toml
[dependencies]
ink_primitives = { git = "https://github.com/paritytech/ink", default-features = false }
ink_metadata = { git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { git = "https://github.com/paritytech/ink", default-features = false }
ink_storage = { git = "https://github.com/paritytech/ink", default-features = false }
ink_lang = { git = "https://github.com/paritytech/ink", default-features = false }
ink_prelude = { git = "https://github.com/paritytech/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }
scale-info = { version = "1", default-features = false, features = ["derive"], optional = true }

# These dependencies
pausable = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
   "ink_primitives/std",
   "ink_metadata",
   "ink_metadata/std",
   "ink_env/std",
   "ink_storage/std",
   "ink_lang/std",
   "scale/std",
   "scale-info",
   "scale-info/std",

   # These dependencies
   "pausable/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `pausable::traits`.

```rust
#[brush::contract]
pub mod my_pausable {
   use pausable::traits::*;
```

## Step 3: Define storage

Declare storage struct and declare the field related to `PausableStorage`.
Then you need to derive `PausableStorage` trait and mark corresponding field
with `#[PausableStorageField]` attribute. Deriving this trait allows you to reuse
the default implementation of `Pausable`.

```rust
#[ink(storage)]
#[derive(Default, PausableStorage)]
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
impl MyFlipper {
   #[ink(constructor)]
   pub fn new() -> Self {
      Self::default()
   }

   #[ink(message)]
   #[brush::modifiers(when_not_paused)]
   pub fn flip(&mut self) {
      self.flipped = !self.flipped;
   }

   #[ink(message)]
   pub fn pause(&mut self) {
      self._pause()
   }

   #[ink(message)]
   pub fn unpause(&mut self) {
      self._unpause()
   }
}

impl Pausable for MyFlipper {}
```