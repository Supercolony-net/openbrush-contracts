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
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "psp1155/std",
   "ownable/std",
   "brush/std",
]
```
2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`.
   Import **everything** from according trait modules.
```rust
#[brush::contract]
pub mod ownable {
   use psp1155::traits::*;
   use ownable::traits::*;
   use brush::{
      modifiers,
   };
```
3. Declare storage struct and derive `PSP1155Storage` and `OwnableStorage`
   traits. Deriving these traits will add required fields to your structure
   for implementation of according traits. Your structure must implement
   `PSP1155Storage` and `OwnableStorage` traits if you want to use the
   default implementation of `IPSP1155` and `IOwnable`.
```rust
#[ink(storage)]
#[derive(Default, PSP1155Storage, OwnableStorage)]
pub struct PSP1155Struct {}
```
4. After that you can inherit implementation of `IPSP1155` and `IOwnable` traits.
   You can customize(override) some methods there.
```rust
impl IOwnable for PSP1155Struct {}
impl IPSP1155 for PSP1155Struct {}
```
5. Now you only need to define constructor to define owner as the contract initiator
   and your basic version of `IPSP1155` contract is ready.
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
6. Let's customize it. We will implement `IPSP1155Mint` trait.
   It will call `only_owner` function inside to verify that caller is the owner.
```rust
impl IPSP1155Mint for PSP1155Struct {
   #[modifiers(only_owner)]
   fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
      // We added modifier to function.
      // #[super]self.mint(to, id, amount) will call default implementation from trait
      #[super]self.mint(to, id, amount);
   }

   #[modifiers(only_owner)]
   fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
      // We added modifier to function.
      // #[super]self.burn(from, id, amount) will call default implementation from trait
      #[super]self.burn(from, id, amount);
   }
}
```
