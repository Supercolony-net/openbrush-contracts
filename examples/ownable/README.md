## Overview
This example shows how you can use the implementation of
[access-control](contracts/access/ownable) and
[psp1155](contracts/token/psp1155) together to provide only_owner rights to mint and burn NFT tokens.

## Steps
1. You need to include `psp1155`, `ownable` and `brush` in cargo file.
```markdown
[dependencies]
...

psp1155 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
ownable = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
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
   Import traits, errors, macros and structs which you want to use.
```rust
#[brush::contract]
pub mod ownable {
    use psp1155::{
        traits::{IPSP1155, Id, IPSP1155MetadataURI, IPSP1155Mint},
        impls::{PSP1155Storage, PSP1155MetadataURIStorage, PSP1155, PSP1155MetadataURI, PSP1155Mint}
    };
    use ownable::{
        traits::{IOwnable, OwnableError},
        impls::{OwnableStorage, Ownable}
    };
    use brush::{
        traits::{InkStorage}
    };
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
    };
    use ink_prelude::{
        string::{String},
        vec::Vec,
    };
```
3. Declare storage struct and derive `PSP1155Storage`, `PSP1155MetadataURIStorage` and `OwnableStorage`
   traits. Deriving these traits will add required fields to your structure
   for implementation of according traits. Your structure must implement
   `PSP1155Storage`, `PSP1155MetadataURIStorage` and `OwnableStorage` traits if you want to use the
   default implementation of `PSP1155` and `Ownable`.
```rust
    #[ink(storage)]
    #[derive(Default, PSP1155Storage, PSP1155MetadataURIStorage, OwnableStorage)]
    pub struct PSP1155Struct {}
```
4. After that you can inherit implementation of `PSP1155` and `Ownable` traits.
   You can customize(override) some methods there.
```rust
    // InkStorage is a utils trait required by any Storage trait
    impl InkStorage for PSP1155Struct {}
    impl Ownable for PSP1155Struct {}
    impl PSP1155 for PSP1155Struct {}
```
5. Now you have all basic logic of `PSP1155` and `Ownable` on rust level.
   But all methods are internal now (it means that anyone can't call these methods from outside of contract).
   If you want to make them external you MUST derive `IPSP1155` and `IOwnable` traits.
   Deriving of these traits will generate external implementation of all methods from `IPSP1155` and `IOwnable`.
   Macro will call the methods with the same name from `PSP1155` and `Ownable` traits.
```rust
    #[ink(storage)]
    #[derive(Default, PSP1155Storage, PSP1155MetadataURIStorage, OwnableStorage, IPSP1155, IOwnable)]
    pub struct PSP1155Struct {}
```
6. Now you only need to define constructor to define owner as the cotnract instanciator
   and your basic version of `PSP1155` contract is ready.
```rust
    impl PSP1155Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_owner(caller);
        instance
    }

    #[inline]
    fn only_owner(&self) {
        assert_eq!(self._owner(), &self.env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
    }
}
```
7. Let's customize it. We will implement `IPSP1155Mint` trait. For that we need inherit `PSP1155Mint`.
   It will call `only_owner` function inside to verify that caller is the owner.
```rust
    impl PSP1155Mint for PSP1155Struct {}
    impl IPSP1155Mint for PSP1155Struct {
    #[ink(message)]
    fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
        self.only_owner();
        PSP1155Mint::mint(self, to, id, amount);
    }

    #[ink(message)]
    fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
        self.only_owner();
        PSP1155Mint::burn(self, from, id, amount);
    }
}
```

8. You can aslo add custome uri to your contract. Fot this you can implement `IPSP1155MetadataURI` trait.
```rust
    impl PSP1155MetadataURI for PSP1155Struct {}
    impl IPSP1155MetadataURI for PSP1155Struct {
    #[ink(message)]
    fn uri(&self, _id: Id) -> Option<String> {
        self._uri().clone()
    }
}
```