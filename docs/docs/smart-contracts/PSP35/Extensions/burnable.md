---
sidebar_position: 3
title: PSP37 Burnable
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37/extensions/burnable.rs) extension.

## How to use this extension

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

For your smart contract to use this extension, you only need to implement the 
`PSP37Burnable` trait in your `PSP37` smart contract. Add import for 
`openbrush::contracts::psp37::extensions::burnable::*`, inherit the implementation for 
`PSP37Burnable` trait, where you can also customize (override) the original functions 
from `PSP37Burnable`.

```rust
use openbrush::contracts::psp37::extensions::burnable::*;

impl PSP37Burnable for Contract {}
```

And that's it! Your `PSP37` is now extended by the `PSP37Burnable` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Burnable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp37_extensions/burnable).
 the documentation for the basic implementation of [PSP37](/smart-contracts/PSP37).