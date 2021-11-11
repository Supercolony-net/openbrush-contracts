#![cfg_attr(not(feature = "std"), no_std)]

pub mod macros;
pub mod traits;

// Each proc macro here is doing some kind of dark magic, be careful, you can summon devil=)
pub use proc_macros::{
    blake2b_256,
    blake2b_256_as_u32,
    contract,
    modifier_definition,
    modifiers,
    trait_definition,
    wrapper,
};
pub use test_utils;
