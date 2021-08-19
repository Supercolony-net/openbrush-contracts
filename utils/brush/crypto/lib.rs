#![cfg_attr(not(feature = "std"), no_std)]

pub mod ecdsa;

#[allow(unused_imports)]
use ink_allocator::*;
#[allow(unused_imports)]
use ink_env::*;

pub fn hash_keccak_256(input: &[u8]) -> [u8; 32] {
    let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
    ink_env::hash_bytes::<hash::Keccak256>(input, &mut output);
    output
}

pub fn hash_blake2b_256(input: &[u8]) -> [u8; 32] {
    let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
    ink_env::hash_bytes::<hash::Blake2x256>(input, &mut output);
    output
}