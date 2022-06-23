---
sidebar_position: 1
title: Overview
---

This doc contains description of how the OpenBrush library can be imported and used. 

The OpenBrush is using ink! stable release `v3.0.0` branch at the moment.
So you should use the same version of the ink! across your project.

#### The default `toml` of your project with OpenBrush:
```toml
[dependencies]
# Import of all ink! crates
ink_primitives = { version = "~3.3.0", default-features = false }
ink_metadata = { version = "~3.3.0", default-features = false, features = ["derive"], optional = true }
ink_env = { version = "~3.3.0", default-features = false }
ink_storage = { version = "~3.3.0", default-features = false }
ink_lang = { version = "~3.3.0", default-features = false }
ink_prelude = { version = "~3.3.0", default-features = false }
ink_engine = { version = "~3.3.0", default-features = false, optional = true }

scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2", default-features = false, features = ["derive"], optional = true }

# Brush dependency
openbrush = { version = "~2.1.0", default-features = false }

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

  # Brush dependency
  "openbrush/std",
]
ink-as-dependency = []
```

To avoid unexpected compilation errors better to always import all ink! crates.

By default, the `openbrush` crate provides [macros](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs)
for simplification of the development and [traits](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/traits) of 
contracts(you can implement them by yourself, and you can use them for a cross-contract calls). 

The OpenBrush also provides the default implementation of traits that can be enabled via crate features. 
A list of all available features you can find [here](https://github.com/Supercolony-net/openbrush-contracts/blob/main/Cargo.toml#L36).
The default implementation of traits requires the usage of the unstable feature [min-specialization](https://doc.rust-lang.org/beta/unstable-book/language-features/min-specialization.html).
You can enable it by adding `#![feature(min_specialization)]` at the top of your root module(for more information check [rust official documentation](https://doc.rust-lang.org/rustdoc/unstable-features.html)). 

> **_Note:_**  ink! requires put `#![cfg_attr(not(feature = "std"), no_std)]` at the top of root crate.

> **_Note:_**  Some default implementations for traits provide additional methods that can be overridden. 
> These methods are defined in a separate internal trait. It has the name of the original trait + suffix `Internal`. 
> If you want to override them you need to do that in the impl section of the internal trait.

Also, that doc contains links to the examples of how to reuse and customize the default implementation of traits.

* [PSP22](PSP22/psp22.md) is an example of how you can reuse the implementation of
  [psp22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22). You also can find examples of how to reuse extensions.
  * [PSP22Metadata](PSP22/Extensions/metadata.md): metadata for PSP22.
  * [PSP22Mintable](PSP22/Extensions/mintable.md): creation of new tokens.
  * [PSP22Burnable](PSP22/Extensions/burnable.md): destruction of own tokens.
  * [PSP22Wrapper](PSP22/Extensions/wrapper.md): wrapper for PSP22 token (useful for governance tokens etc.).
  * [PSP22FlashMint](PSP22/Extensions/flashmint.md): extension which allows performing flashloans of the token by minting and burning the token.
  * [PSP22Pausable](PSP22/Extensions/pausable.md): example of using pausable extension in the PSP22 contract.
  * [PSP22Capped](PSP22/Extensions/capped.md): extension which adds a cap for total supply of PSP22 tokens.
  * [PSP22TokenTimelock](PSP22/Utils/token-timelock.md): Utility which allows token holders to lock their tokens for a specified amount of time.
* [PSP34](PSP34/psp34.md) is an example of how you can reuse the implementation of
  [psp34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34). You also can find examples of how to reuse extensions.
  * [PSP34Metadata](PSP34/Extensions/metadata.md): metadata for PSP34.
  * [PSP34Mintable](PSP34/Extensions/mintable.md): creation of new tokens.
  * [PSP34Burnable](PSP34/Extensions/burnable.md): destruction of own tokens.
  * [PSP34Enumerable](PSP34/Extensions/enumerable.md): iterating over contract's tokens.
* [PSP35](PSP35/psp35.md) is an example of how you can reuse the implementation of
  [psp35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35). You also can find examples of how to reuse extensions.
  * [PSP35Metadata](PSP35/Extensions/metadata.md): metadata for PSP35.
  * [PSP35Mintable](PSP35/Extensions/mintable.md): creation of new tokens.
  * [PSP35Burnable](PSP35/Extensions/burnable.md): destruction of own tokens.
  * [PSP35Batch](PSP35/Extensions/batch.md): batch transferring of tokens.
  * [PSP35Enumerable](PSP35/Extensions/enumerable.md): iterating over contract's tokens.
* [Access Control](access-control.md) shows how you can use the implementation of
  [access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/access/access_control) and
  [psp34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) together to provide rights to mint and burn NFT tokens.
  * [AccessControlEnumerable](PSP34/Extensions/enumerable.md): iterating over contract's tokens.
* [Ownable](ownable.md) shows how you can use the implementation of
  [ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable) and
  [psp35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) together to provide rights to mint and burn tokens.
* [ReentrancyGuard](reentrancy-guard.md) shows how you can use the implementation of
  [non_reentrant](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard)
  modifier to prevent reentrancy during certain functions.
* [Pausable](pausable.md) shows how you can use the implementation of
  [pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/pausable)
  contract and modifiers.
* [TimelockController](timelock-controller.md) shows how you can use the implementation of
  [timelock-controller](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller)
  to execute a transaction with some delay via governance.
* [PaymentSplitter](payment-splitter.md) shows how you can use the implementation of
  [payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/finance/payment_splitter)
  to split received native tokens between participants of the contract.
* [Diamond](diamond.md) shows how you can use the implementation of
  [diamond](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradability/diamond)
  pattern to split your contract into small parts and support upgradability.
  * [DiamondLoupe](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradability/diamond/extensions): iterating over contract's facets.
* [Proxy](proxy.md) shows how you can use the implementation of
  [proxy](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradability/proxy)
  pattern to support upgradability of your contract.