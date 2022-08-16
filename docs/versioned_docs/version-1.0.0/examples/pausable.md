## Overview

This example shows how you can reuse the implementation of
[pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable) in `Flipper` contract to `flip` only if the contract is not paused.

## Steps

1. Include dependencies to `pausable` and `brush` in the cargo file.

```markdown
[dependencies]
ink_primitives = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }
scale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }

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

2. Replace `ink::contract` macro by `brush::contract`.
   Import **everything** from `pausable::traits`.

```rust
#[brush::contract]
pub mod my_pausable {
   use pausable::traits::*;
```

3. Declare storage struct and declare the field related to `PausableStorage`
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

4. Inherit the implementation of `Pausable`. You can customize (override) methods in this `impl` block.

```rust
impl Pausable for MyFlipper {}
```

5. Define constructor. Your basic version of `Pausable` contract is ready!

```rust
impl MyFlipper {
   #[ink(constructor)]
   pub fn new() -> Self {
      Self::default()
   }
}
```

6. Customize it by adding flipper logic. We will implement `flip` method marked with `when_not_paused` modifier.

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