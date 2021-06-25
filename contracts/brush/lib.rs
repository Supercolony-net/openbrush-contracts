#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;
pub use proc_macros::{
    contract,
    trait_definition,
    internal_trait_definition,
    modifiers,
};