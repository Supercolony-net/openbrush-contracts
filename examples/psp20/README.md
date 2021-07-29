## Overview

This example shows how you can reuse the implementation of
[psp20](contracts/token/psp20) token(by the same way you can reuse
[psp721](contracts/token/psp721) and [psp1155](contracts/token/psp1155)). Also, this example shows how you can customize
the logic, for example, to not allow transfer tokens to `hated_account`.

## Steps

1. You need to include `psp20` and `brush` in cargo file.

```markdown
[dependencies]
...

psp20 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "psp20/std",
   "brush/std",
]
```

2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`. Import **everything**
   from `psp20` trait module.

```rust
#[brush::contract]
pub mod my_psp20 {
    use psp20::traits::*;
    use ink_storage::Lazy;
    use ink_prelude::{string::String, vec::Vec};
```

3. Declare storage struct and declare the fields related to `PSP22Storage` and `PSP22MetadataStorage`
   traits. Then you need to derive `PSP22Storage` and `PSP22MetadataStorage` traits and mark according fields
   with `#[PSP22StorageField]` and `#[PSP22MetadataStorageField]` attributes. Deriving these traits allow you to reuse
   the default implementation of `PSP22` and `PSP22Metadata`.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PSP22MetadataStorage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp20: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

4. After that you can inherit implementation of `PSP22` and `PSP22Metadata` traits. You can customize(override) some
   methods there.

```rust
impl PSP22 for MyPSP22 {}

impl PSP22Metadata for MyPSP22 {}
```

5. Now you only need to define constructor and your basic version of `PSP22` contract is ready.

```rust
impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
        let mut instance = Self::default();
        instance.metadata.name = Lazy::new(name);
        instance.metadata.symbol = Lazy::new(symbol);
        instance.metadata.decimals = Lazy::new(decimal);
        instance._mint(instance.env().caller(), _total_supply);
        instance
    }
}
```

6. Let's customize it. It will contain two public methods `set_hated_account` and `get_hated_account`. Also we will
   override `_before_token_transfer` method in `PSP22` implementation. And we will add a new field to structure
   - `hated_account: AccountId`

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PSP22MetadataStorage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp20: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
    // fields for hater logic
    hated_account: AccountId,
}

impl PSP22 for MyPSP22 {
    // Let's override method to reject transactions to bad account
    fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {
        assert!(_to != self.hated_account, "{}", PSP22Error::Custom(String::from("I hate this account!")).as_ref());
    }
}

impl PSP22Metadata for MyPSP22 {}

impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
        let mut instance = Self::default();
        instance.metadata.name = Lazy::new(name);
        instance.metadata.symbol = Lazy::new(symbol);
        instance.metadata.decimals = Lazy::new(decimal);
        instance._mint(instance.env().caller(), _total_supply);
        instance
    }

    #[ink(message)]
    pub fn set_hated_account(&mut self, hated: AccountId) {
        self.hated_account = hated;
    }

    #[ink(message)]
    pub fn get_hated_account(&self) -> AccountId {
        self.hated_account.clone()
    }
}
```
