use openbrush::contracts::traits::psp22::{
    extensions::metadata::*,
    extensions::mintable::*,
    *,
};

#[openbrush::wrapper]
pub type StableCoinRef = dyn PSP22 + PSP22Metadata + PSP22Mintable;

#[openbrush::trait_definition]
pub trait StableCoin: PSP22 + PSP22Metadata + PSP22Mintable {}
