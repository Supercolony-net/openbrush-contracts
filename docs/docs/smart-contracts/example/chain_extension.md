---
sidebar_position: 11
title: Chain extension
---

The library provides tools and primitives to simplify the development of chain extensions for ink! and for the substrate.

## OpenBrush Chain Extension library

The [OBCE](https://github.com/Supercolony-net/obce) library provides macros that allow implementing the same trait on ink! and substrate. 
Macros generate all logic related to encoding and decoding arguments, calculation of functions id and extension id, 
matching function ids, and returning errors. On the substrate side, you need to implement the trait and write the logic of each method. 
The ink! side will already be prepared to pass all data to you.


### Definition macro
To use library you need add it as crate in `Cargo.toml`.
```toml
obce = { git = "https://github.com/Supercolony-net/obce", default-features = false }
```

We define `trait` module and will use it for traits definition.
```rust
pub mod traits{}

```

To create your own chain extension you need to describe types that will be used in chain extension definition.
```rust
pub trait Environment {
    type AccountId;
    type AssetId;
    type Balance;
}
```

To create your own pallet assets trait you need to use definition macro.
```rust
#[obce::definition]
pub trait PalletAssets<T: Environment> {}
```

Also you can specify pallet assets id by using `id` parameter. It will change
`obce::codegen::ExtensionDescription::ID` field.

```rust
#[obce::definition(id = 0x01)]
```

You can implement origin of the call and add it as parameter for function. In this case the smart contract can execute methods on behalf of the `caller` or itself.
```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[cfg_attr(
    feature = "ink",
    derive(ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout,)
)]
#[cfg_attr(all(feature = "ink", feature = "std"), derive(ink_storage::traits::StorageLayout))]
pub enum Origin {
    Caller,
    Address,
}

impl Default for Origin {
    fn default() -> Self {
        Self::Address
    }
}

#[cfg(feature = "ink")]
impl ink_storage::traits::SpreadAllocate for Origin {
    fn allocate_spread(_ptr: &mut ink_primitives::KeyPtr) -> Self {
        Self::Address
    }
}

#[obce::definition]
pub trait PalletAssets<T: Environment> {
    fn transfer(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        target: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>>;
}
```
To use this approach you need these features:
```toml
[features]
ink = [
	"obce/ink",
	"ink_primitives",
	"ink_storage",
]
std = [
	"scale-info/std",
	"scale/std",
]
```

### Ink implementation
First let's create a separate module for it and use `ink` feature (we have already added it in `Cargo.toml` in previous step).
```rust
#[cfg(feature = "ink")]
pub mod ink {}
```
Then we need to import types from library and from our traits module.
```rust
use obce::ink::ink_env::{
    DefaultEnvironment,
    Environment,
};
use crate::traits::{
    Environment as AssetsEnvironment,
    PalletAssets,
};
```
The next step is creating assets extension.
```rust
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout, ink_storage::traits::SpreadAllocate)]
#[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
pub struct AssetsExtension;
```
Using library we implement trait with chain extension types.
```rust
impl<T: Environment> AssetsEnvironment for T {
    type AccountId = T::AccountId;
    type AssetId = u32;
    type Balance = T::Balance;
}
```
Finally, we implement PalletAssets trait for AssetsExtension.
```rust
impl AssetsExtension {
    pub fn new() -> Self {
        Self {}
    }
}

impl PalletAssets<DefaultEnvironment> for AssetsExtension {}
```

### Substrate implementation
You need `#[obce::implementation]` macro when you want to implement your pallet assets trait.
Also you need to add [Substrate](https://github.com/paritytech/substrate) as dependency in Cago.toml.
```toml
pallet-assets = { git = "https://github.com/paritytech/substrate", default-features = false, optional = true }
```

First step is adding imports.
```rust
use obce::substrate::{
    frame_system::Config as SysConfig,
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
};
use pallet_assets::Config as AssetConfig;
```

The next step is creating assets extension.
```rust
#[derive(Default)]
pub struct AssetsExtension;
```

Using library we implement trait with chain extension types.
```rust
impl<T: SysConfig + AssetConfig + ContractConfig> AssetsEnvironment for T {
    type AccountId = <T as SysConfig>::AccountId;
    type AssetId = <T as AssetConfig>::AssetId;
    type Balance = <T as AssetConfig>::Balance;
}
```

Implementation of `PalletAssets` trait.
```rust
#[obce::implementation]
impl<'a, 'b, E, T> PalletAssets<T> for ExtensionContext<'a, 'b, E, T, AssetsExtension>
where
    T: SysConfig + AssetConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
    <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
{}
```

We can call [OpenBrush Chain Extension library methods](https://github.com/Supercolony-net/obce/blob/main/codegen/src/implementation.rs) on `pallet_assets::Pallet::<T>` trait.
```rust
fn transfer(
    &mut self,
    origin: Origin,
    id: T::AssetId,
    target: T::AccountId,
    amount: T::Balance,
) -> Result<(), Error<T>> {
    Ok(pallet_assets::Pallet::<T>::transfer(
        self.select_origin(origin)?,
        id,
        target.into(),
        amount,
    )?)
}
```
