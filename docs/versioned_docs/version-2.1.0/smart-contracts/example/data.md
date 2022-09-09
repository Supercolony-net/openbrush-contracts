---
sidebar_position: 6
title: Data and derive macro
---

## Data segregation

Rust doesn't have inheritance like OOP languages.
If you want to "inherit" some fields, you can use structural composition.
If you want to "inherit" some implementation, you can use traits. Traits can have a [default implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations) or a [generic implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods).
The traits in Rust can't contain fields, it is pure interfaces.

Based on that information we propose you the following concept of smart contract development:

### Storage trait

Extract the logic of data storing into a separate trait to have the ability to define the default implementation without knowing what contract will inherit that. That trait can have a simple naming like `NAME_OF_ORIGINAL_TRAIT` + `Storage` suffix.

```rust
pub trait PointStorage {
    fn get(&self) -> & PointData;
    fn get_mut(&mut self) -> &mut PointData;
}
```

### Data of the trait

That trait returns some data with fields that can be used in the implementation. The data is a simple struct with fields. Later that struct can be embedded into the contract struct.
```rust
pub struct PointData {
    pub x: u32,
    pub y: u32,
}
```

### Default implementation

Define the default or generic implementation for your main trait with the restriction that `Self` should also implement storage trait.

A default implementation:
```rust
pub trait Point: PointStorage {
    fn x(&self) -> u32 {
        PointStorage::get(self).x
    }
    
    fn y(&self) -> u32 {
        PointStorage::get(self).y
    }
    
    fn name(&self) -> String {
        "AlphaPoint".to_string()
    }
}
```
or a generic implementation:
```rust
#![feature(min_specialization)]

pub trait Point {
    fn x(&self) -> u32;

    fn y(&self) -> u32;

    fn name(&self) -> String;
}

impl<T: PointStorage> Point for T {
    default fn x(&self) -> u32 {
        PointStorage::get(self).x
    }

    default fn y(&self) -> u32 {
        PointStorage::get(self).y
    }

    default fn name(&self) -> String {
        "AlphaPoint".to_string()
    }
}
```

### "Inheritance" of the implementation

When someone wants to "inherit" implementation and fields, he can embed the data structure, implement the storage trait, and define an impl section of the main trait:
```rust
struct PointContract {
    point: PointData,
}

impl PointStorage for PointContract {
    fn get(&self) -> & PointData {
        &self.point
    }
    fn get_mut(&mut self) -> &mut PointData {
        &mut self.point
    }
}

impl Point for PointContract {}
```

## Macros from OpenBrush

Some macros from OpenBrush allows to remove boilerplate code and simplify the development:
- [`declare_storage_trait!`](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs)
macro define the storage like described in the [Storage trait](/smart-contracts/example/data#storage-trait)
```rust
use openbrush::declare_storage_trait;
declare_storage_trait!(PointStorage);
```
- [`impl_storage_trait!`](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/macro/src/lib.rs)
macro implements the storage trait for the contract and return the field from that contract of the data type
```rust
use openbrush::impl_storage_trait;
impl_storage_trait!(PointStorage, PointContract, point, PointData);
```
- Also, if you are familiar with [derive](https://doc.rust-lang.org/rust-by-example/trait/derive.html) macro:

You can create a derive macro for your storage trait by yourself with 
[`declare_derive_storage_trait!`](https://github.com/Supercolony-net/openbrush-contracts/blob/main/lang/src/derive.rs)
from OpenBrush.
To define a derive macro you need a separate directory(let's call it `derive`). 
This directory contains the standard stuff of a Cargo folder - `.gitignore`, `Cargo.toml`, and the `lib.rs` file,
inside of which we will define a derive. So in the end, our `lib.rs` file will 
look like this:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use openbrush::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_point_storage, PointStorage, PointStorageField);
```

In the `Cargo.toml` of the derive folder you need to import `openbrush` dependencies:

```toml
[dependencies]
syn = { version = "1.0" }
quote = "1.0"
proc-macro2 = "1"
openbrush = { version = "~2.1.0", default-features = false }

[lib]
name = "point_derive"
path = "lib.rs"
proc-macro = true

[features]
default = ["std"]
std = []
```

After importing that derive crate into your main contract, 
you can use `derive(PointStorage)` instead of `impl_storage_trait!`.
```rust
use point_derive::PointStorage;

#[derive(PointStorage)]
struct PointContract {
    #[PointStorageField]
    point: PointData,
}

impl Point for PointContract {}
```