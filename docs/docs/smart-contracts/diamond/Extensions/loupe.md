---
sidebar_position: 2
title: Diamond Loupe
---

This example shows how you can reuse the implementation of [Diamond Standard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradeability/diamond) with [Diamond Loupe](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/upgradeability/diamond/extensions/diamond_loupe.rs) extension, which allows you to iterate over diamond contract's facets and available functions.

## How to use this extension

First, you should implement basic version of [Diamond standard](/smart-contracts/diamond).

For your smart contract to use this extension, you only need to implement the `DiamoundLoupe` trait in your
`Diamond` smart contract. Add import for `openbrush::contracts::diamond::extensions::diamond_loupe::*`,
inherit the implementation for `DiamondLoupe` trait, where you can also customize (override)
the original functions from `DiamondLoupe`.

```rust
use openbrush::contracts::diamond::extensions::diamond_loupe::*;

impl DiamondLoupe for Contract {}
```

And that's it! Your `Diamond` is now extended by the `DiamondLoupe` extension and ready to use its functions!
You can check an example of the usage of [Diamond Loupe](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/diamond).
