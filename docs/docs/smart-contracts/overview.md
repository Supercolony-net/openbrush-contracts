---
sidebar_position: 1
title: Overview
---

This doc contains description of how the OpenBrush library can be imported and used. 

The OpenBrush is using ink! stable release `3.3.0` at the moment.
So you should use the same version of the ink! across your project.
If you use an old version of ink, you need to use the old version of OpenBrush.
OpenBrush had several significant changes in API, so you check the [Wizard](https://openbrush.io)
to study how to use different versions of OpenBrush.

The documentation describes the latest available OpenBrush and how to use it.
It doesn't contain [versioning](https://github.com/Supercolony-net/openbrush-contracts/issues/127) yet.

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

To avoid unexpected compilation errors better to always import all ink! crates until resolving
[issue](https://github.com/paritytech/ink/issues/825).

By default, the `openbrush` crate provides [macros](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs)
for simplification of the development and [traits](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/traits) of 
contracts(you can implement them by yourself, and you can use them for a cross-contract calls). 

The OpenBrush also provides the default implementation of traits that can be enabled via crate features. 
A list of all available features you can find [here](https://github.com/Supercolony-net/openbrush-contracts/blob/main/Cargo.toml#L51).
The default implementation of traits requires the usage of the unstable feature [min-specialization](https://doc.rust-lang.org/beta/unstable-book/language-features/min-specialization.html).
You can enable it by adding `#![feature(min_specialization)]` at the top of your root module(for more information check [rust official documentation](https://doc.rust-lang.org/rustdoc/unstable-features.html)). 

> **_Note:_**  ink! requires put `#![cfg_attr(not(feature = "std"), no_std)]` at the top of root crate.

> **_Note:_**  Some default implementations for traits provide additional methods that can be overridden. 
> These methods are defined in a separate internal trait. It has the name `Internal`. 
> If you want to override them you need to do that in the impl section of the internal trait.
> If you imported several internal traits, you could specify which one you want to use, `psp22::Internal` or `psp34::Internal`.

#### Reuse implementation of traits from OpenBrush

The doc contains links to the examples of how to reuse and customize the default implementation of traits.

All default implementations of the traits provided by OpenBursh have the same pattern. 
Consequently, the re-usage of each implementation in your contract also has the same pattern.

Each implementation of the contract has its module and its feature that enables that 
module. A list of available modules you can find [here](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/lib.rs#L33), 
a list of available features [here](https://github.com/Supercolony-net/openbrush-contracts/blob/main/Cargo.toml#L51). 
Each module can be reached via the `openbrush::contracts::` namespace. For example, 
to use the `psp22` module, you need to import `openbrush::contracts::psp22`; 
to use the `ownable` module, you need to import `openbrush::contracts::ownable`.

Before importing each module, first you need to enable the corresponding feature in your `Cargo.toml`.
The name of the feature is the same as the name of the module. For example:

To enable `psp22`:
```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp22"] }
```

To enable `ownable`:
```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["ownable"] }
```

To enable both:
```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp22", "ownable"] }
```

After enabling the feature and importing the corresponding module, you need to embed the module 
data structure into your contract as a field and implement the `openbrush::traits::Storage` 
trait for that field. In most cases, the data structure of each module is named `Data`. 
If importing several modules, you can specify which data you want to use via namespaces like 
`psp22::Data` or `ownable::Data`.

Embedding of data structures looks like:
```rust
use openbrush::contracts::ownable::*;
use openbrush::contracts::psp22::*;

#[ink(storage)]
pub struct Contract {
    foo: psp22::Data,
    bar: ownable::Data,
}
```

Each contract that wants to reuse implementation should implement the 
`openbrush::traits::Storage` with the corresponding data structure. 
The easiest way to implement that trait is via the derive macro by adding 
`#[derive(Storage)]` and marking the corresponding fields with the `#[storage_field]` 
attribute.

```rust
use openbrush::contracts::ownable::*;
use openbrush::contracts::psp22::*;
use openbrush::traits::Storage;

#[ink(storage)]
#[derive(Storage)]
pub struct Contract {
    #[storage_field]
    foo: psp22::Data,
    #[storage_field]
    bar: ownable::Data,
}
```

Now your contract has access to default implementation on the Rust level. 
It is on the Rust level so you can call methods only inside your contract
(in the example, it is methods of `PSP22`, `psp22::Internal`, `Ownable`, and 
`ownable::Internal` traits). If you want to make all methods of some trait public, 
you should explicitly implement the corresponding trait. For example:

```rust
use openbrush::contracts::ownable::*;
use openbrush::contracts::psp22::*;
use openbrush::traits::Storage;

#[ink(storage)]
#[derive(Storage)]
pub struct Contract {
    #[storage_field]
    foo: psp22::Data,
    #[storage_field]
    bar: ownable::Data,
}

impl PSP22 for Contract {}
impl Ownable for Contract {}
```

Remember, only traits with `#[ink(message)]` methods can be public. `psp22::Internal` 
and `ownable::Internal` can't be exposed. It is for internal usage only.

The implementation in OpenBrush is called "default" because you can customize(override) it. 
You can override one method, or several, as you wish. For example:

```rust
use openbrush::contracts::ownable::*;
use openbrush::contracts::psp22::*;
use openbrush::traits::Storage;

#[ink(storage)]
#[derive(Storage)]
pub struct Contract {
    #[storage_field]
    foo: psp22::Data,
    #[storage_field]
    bar: ownable::Data,
}

impl PSP22 for Contract {
    fn balance_of(&self, owner: AccountId) -> Balance {
        // For example you can break `balance_of` method and return always zero
        return 0
    }
}

impl Ownable for Contract {
    fn owner(&self) -> AccountId {
        // For example you can return always zero owner
        openbrush::traits::ZERO_ADDRESS.into()
    }
}

impl psp22::Internal for Contract {
  fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
    return Err(PSP22Error::Custom("I don't want to mint anything".to_string()));
  }
}

impl ownable::Internal for Contract {
  fn _init_with_owner(&mut self, owner: AccountId) {
    // Maybe you want to change something during initialization of the owner
  }
}
```

Work with each module has the same pattern. The difference is only in the naming of 
the module and main trait. Some contract extensions require additional steps, so below, 
you can find instructions on how to work with them:

* [PSP22](PSP22/psp22.md) is an example of how you can reuse the implementation of
  [psp22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22). You also can find examples of how to reuse extensions.
  * [PSP22Metadata](PSP22/Extensions/metadata.md): metadata for PSP22.
  * [PSP22Mintable](PSP22/Extensions/mintable.md): creation of new tokens.
  * [PSP22Burnable](PSP22/Extensions/burnable.md): destruction of own tokens.
  * [PSP22Wrapper](PSP22/Extensions/wrapper.md): wrapper for PSP22 token (useful for governance tokens etc.).
  * [PSP22FlashMint](PSP22/Extensions/flashmint.md): extension which allows performing flashloans of the token by minting and burning the token.
  * [PSP22Pausable](PSP22/Extensions/pausable.md): example of using pausable extension in the PSP22 contract.
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
* [ReentrancyGuard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard)
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
  [diamond](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradeability/diamond)
  pattern to split your contract into small parts and support upgradeability.
  * [DiamondLoupe](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradeability/diamond/extensions): iterating over contract's facets.
* [Proxy](proxy.md) shows how you can use the implementation of
  [proxy](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradeability/proxy)
  pattern to support upgradeability of your contract.