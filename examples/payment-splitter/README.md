## Overview

This example shows how you can reuse the implementation of
[payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment-splitter).

## Steps

1. Include dependencies `payment-splitter` and `brush` in cargo file.

```markdown
[dependencies]
...

payment-splitter = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }


# payment-splitter uses dividing inside, so your version of rust can require you to disable check overflow.
[profile.dev]
overflow-checks = false

[profile.release]
overflow-checks = false

[features]
default = ["std"]
std = [
 ...
   
   "payment-splitter/std",
   "brush/std",
]
```

2. Replace `ink::contract` macro by `brush::contract`.
   Import **everything** from corresponding trait modules.

```rust
#[brush::contract]
pub mod my_payment_splitter {
   use payment_splitter::traits::*;
   use ink_prelude::vec::Vec;
```

3. Declare storage struct and declare the field related to `PaymentSplitterStorage`
   Then you need to derive `PaymentSplitterStorage` trait and mark corresponsing field
   with `#[PaymentSplitterStorageField]` attribute. Deriving this trait allows you to reuse
   the default implementation of `PaymentSplitter`.

```rust
#[ink(storage)]
#[derive(Default, PaymentSplitterStorage)]
pub struct SplitterStruct {
   #[PaymentSplitterStorageField]
   splitter: PaymentSplitterData,
}
```

4. Inherit the implementation of `PaymentSplitter`. You can customize (override) methods in this `impl` block.

```rust
impl PaymentSplitter for SplitterStruct {}
```

5. Define constructor and your basic version of `PaymentSplitter` contract is ready.

```rust
impl SplitterStruct {
   #[ink(constructor)]
   pub fn new(payees: Vec<AccountId>, shares: Vec<Balance>) -> Self {
      let mut instance = Self::default();
      instance._init(payees, shares);
      instance
   }
}
```