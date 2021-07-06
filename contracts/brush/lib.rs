#![cfg_attr(not(feature = "std"), no_std)]

pub mod traits;
// Each proc macro here is doing some kind of dark magic, be careful, you can summon devil=)
pub use proc_macros::{
    contract,
    trait_definition,
    storage_trait,
    modifier_definition,
    modifiers,
};
pub use test_utils;