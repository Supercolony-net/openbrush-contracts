#![cfg_attr(not(feature = "std"), no_std)]

pub mod macros;
pub mod traits;

// Each proc macro here is doing some kind of dark magic, be careful, you can summon devil=)
pub use proc_macros::{
    contract,
    modifier_definition,
    modifiers,
    trait_definition,
};
pub use test_utils;
