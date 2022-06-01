use openbrush::contracts::traits::psp22::{
    extensions::metadata::*,
    *,
};

#[openbrush::wrapper]
pub type StableCoinRef = dyn PSP22 + PSP22Metadata;

#[openbrush::trait_definition]
pub trait StableCoin: PSP22 + PSP22Metadata {}
