use openbrush::contracts::traits::{
    ownable::*,
    psp22::{
        extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
        *,
    },
};

#[openbrush::wrapper]
pub type SharesRef = dyn PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable;

#[openbrush::trait_definition]
pub trait Shares: PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable {}
