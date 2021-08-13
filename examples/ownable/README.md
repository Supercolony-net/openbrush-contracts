## Overview

This example shows how you can use the implementation of
[access-control](contracts/access/ownable) and
[psp1155](contracts/token/psp1155) together to provide `only owner` rights to mint and burn tokens.

## Steps

1. You need to include `psp1155`, `ownable` and `brush` in cargo file.

```markdown
[dependencies]
...

psp1155 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
ownable = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "psp1155/std",
   "ownable/std",
   "brush/std",
]
```

2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`. Import **everything**
   from according trait modules.

```rust
#[brush::contract]
pub mod ownable {
   use psp1155::traits::*;
   use ownable::traits::*;
   use brush::modifiers;
   use ink_prelude::vec::Vec;
```

3. Declare storage struct and declare the fields related to `PSP1155Storage` and `OwnableStorage`
   traits. Then you need to derive `PSP1155Storage` and `OwnableStorage` traits and mark according fields
   with `#[PSP1155StorageField]` and `#[OwnableStorageField]` attributes. Deriving these traits allow you to reuse the
   default implementation of `IPSP1155` and `Ownable`.

```rust
#[ink(storage)]
#[derive(Default, PSP1155Storage, OwnableStorage)]
pub struct PSP1155Struct {
    #[PSP1155StorageField]
    psp1155: PSP1155Data,
    #[OwnableStorageField]
    ownale: OwnableData,
}
```

4. After that you can inherit implementation of `IPSP1155` and `Ownable` traits. You can customize(override) some
   methods there.

```rust
impl Ownable for PSP1155Struct {}
impl IPSP1155 for PSP1155Struct {}
```

5. Now you only need to define constructor to define owner as the contract initiator and your basic version
   of `IPSP1155` contract is ready.

```rust
impl PSP1155Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_owner(caller);
        instance
    }
}
```

6. Let's customize it. We will implement `IPSP1155Mint` trait. It will call `only_owner` function inside to verify that
   caller is the owner.

```rust
impl IPSP1155Mint for PSP1155Struct {
   #[ink(message)]
   #[modifiers(only_owner)]
   fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
      self._mint(to, id, amount);
   }

   #[ink(message)]
   #[modifiers(only_owner)]
   fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
      self._burn(from, id, amount);
   }
}
```
