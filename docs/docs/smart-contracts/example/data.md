---
sidebar_position: 6
title: Data and derive macro
---

## Data segregation

Rust doesn't have inheritance like OOP languages.
If you want to "inherit" some fields, you can use structural composition.
If you want to "inherit" some implementation, you can use traits. 
Traits can have a [default implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations) or a [generic implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods).
The traits in Rust can't contain fields, it is pure interfaces.

Based on that information we propose you the following concept of smart contract 
development:

### Storage trait

Extract the logic of data storing into a separate trait to have the ability to
define the default implementation without knowing what contract will inherit that.
You can use that separate trait as a bound in your generic implementation(below we will describe).

You can define your own storage trait like:
```rust
pub trait PointStorage {
    fn get(&self) -> & PointData;
    fn get_mut(&mut self) -> &mut PointData;
}
```

Or you can use `openbrush::traits::Storage` trait from OpenBrush.

`Storage` is a generic trait, so you can use it to work with different storage.
For example, if in your default implementation you need to have `psp22::extensions::metadata::Data` and `psp22::Data`, 
you can add bounds `T: Storage<metadata::Data> + Storage<psp22::Data>`.
It allows you to work with two independent storage.

`openbrush::traits::Storage` trait requires that the inner data implements the 
`openbrush::traits::OccupyStorage` trait. 
It is because each storage in the smart contract should occupy a unique storage key. 
Overlapping of those keys can cause unexpected bugs. Derive macro provided by 
OpenBrush to simplify the implementation of the `Storage` trait also checks that 
the storage key from the `OccupyStorage ` trait is unique.

### Data of the trait

That trait returns some data with fields that can be used in the implementation. 
The data is a simple struct with fields. Later that struct can be embedded into the contract struct.
```rust
pub struct PointData {
    pub x: u32,
    pub y: u32,
}
```

If you want to use `openbrush::traits::Storage` then you also need to implement `openbrush::traits::OccupyStorage`
with unique storage key.

```rust
pub struct PointData {
    pub x: u32,
    pub y: u32,
}

impl openbrush::traits::OccupyStorage for PointData {
    // You can specify your unique key manually like `0x123` or you can use macro
    const KEY: u32 = openbrush::storage_unique_key!(PointData);
}
```

Also, you can use the `openbrush::upgradeable_storage` macro that implements that trait by default,
and also prepare the storage to be upgradeable.

```rust
#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(PointData))]
pub struct PointData {
    pub x: u32,
    pub y: u32,
}
```

### Default implementation

Define the default or generic implementation for your main trait with the restriction that `Self` 
should also implement storage trait.

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

A default implementation with `openbrush::traits::Storage`:
```rust
pub trait Point: openbrush::traits::Storage<PointData> {
    fn x(&self) -> u32 {
        self.data().x
    }
    
    fn y(&self) -> u32 {
        self.data().y
    }
    
    fn name(&self) -> String {
        "AlphaPoint".to_string()
    }
}
```
or a generic implementation with `openbrush::traits::Storage`:
```rust
#![feature(min_specialization)]

pub trait Point {
    fn x(&self) -> u32;

    fn y(&self) -> u32;

    fn name(&self) -> String;
}

impl<T: openbrush::traits::Storage<PointData>> Point for T {
    default fn x(&self) -> u32 {
        self.data().x
    }

    default fn y(&self) -> u32 {
        self.data().y
    }

    default fn name(&self) -> String {
        "AlphaPoint".to_string()
    }
}
```

### "Inheritance" of the implementation

When someone wants to "inherit" implementation and fields, he can embed the data structure, 
implement the storage trait, and define an impl section of the main trait:
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

If you are using `openbrush::traits::Storage` and your data implements `openbrush::traits::OccupyStorage`
trait, then you can use derive macro to automate the implementation of the trait.
Each field for which you want to implement the `Storage` trait should be marked with `#[storage_field]`.

```rust
use openbrush::traits::Storage;

#[derive(Storage)]
struct PointContract {
    #[storage_field]
    point: PointData,
}

impl Point for PointContract {}
```