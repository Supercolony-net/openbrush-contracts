#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod token;

pub mod traits;
// Implementation of traits above
pub use token::*;
