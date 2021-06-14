#![cfg_attr(not(feature = "std"), no_std)]

pub mod macros;
pub mod traits;
pub use proc_macros::{
    contract,
    impl_trait,
    trait_definition,
};