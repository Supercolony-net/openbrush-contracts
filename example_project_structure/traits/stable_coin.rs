use brush::contracts::traits::psp22::{
    extensions::metadata::*,
    *,
};

#[brush::wrapper]
pub type StableCoinRef = dyn PSP22 + PSP22Metadata;

#[brush::trait_definition]
pub trait StableCoin: PSP22 + PSP22Metadata {}
