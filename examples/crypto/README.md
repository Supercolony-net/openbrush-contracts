## Overview

This example shows how you can reuse the implementation of
[pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable) in `Flipper` contract to `flip` only if the contract is paused.

## Steps

1. Include dependencies `pausable` and `brush` in cargo file.

```markdown
[dependencies]
...

pausable = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "pausable/std",
   "brush/std",
]
```

2. Replace `ink::contract` macro by `brush::contract`.
   Import **everything** from according trait modules.

```rust
#[brush::contract]
pub mod my_pausable {
   use pausable::traits::*;
```

3. Declare storage struct and declare the field related to `PausableStorage`
   Then you need to derive `PausableStorage` trait and mark according field
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

4. Inherit implementation of `Pausable`. You can customize(override) methods in this `impl` block.

```rust
impl Pausable for MyFlipper {}
```

5. Define constructor and your basic version of `Pausable` contract is ready.

```rust
impl MyFlipper {
   #[ink(constructor)]
   pub fn new() -> Self {
      Self::default()
   }
}
```

6. Customize it by adding flipper logic. We will implement `flip` method marked with `when_paused` modifier.

```rust
impl MyFlipper {
   #[ink(constructor)]
   pub fn new() -> Self {
      Self::default()
   }

   #[ink(message)]
   #[brush::modifiers(when_paused)]
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