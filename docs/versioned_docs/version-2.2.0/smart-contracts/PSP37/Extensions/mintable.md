---
sidebar_position: 2
title: PSP37 Mintable
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37/extensions/mintable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

For your smart contract to use this extension, you only need to implement the 
`PSP37Mintable` trait in your `PSP37` smart contract. Add import for 
`openbrush::contracts::psp37::extensions::mintable::*`, inherit the implementation for 
`PSP37Mintable` trait, where you can also customize (override) the original functions 
from `PSP37Mintable`.

```rust6
use openbrush::contracts::psp37::extensions::mintable::*;

impl PSP37Mintable for Contract {}
```

And that's it! Your `PSP37` is now extended by the `PSP37Mintable` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Mintable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp37_extensions/mintable).
