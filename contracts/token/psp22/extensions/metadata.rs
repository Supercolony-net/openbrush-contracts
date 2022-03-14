pub use crate::{
    psp22::*,
    traits::psp22::extensions::metadata::*,
};
use brush::declare_storage_trait;
pub use derive::PSP22MetadataStorage;
use ink_prelude::string::String;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP22MetadataData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP22MetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP22MetadataStorage, PSP22MetadataData);

impl<T: PSP22MetadataStorage> PSP22Metadata for T {
    default fn token_name(&self) -> Option<String> {
        self.get().name.clone()
    }

    default fn token_symbol(&self) -> Option<String> {
        self.get().symbol.clone()
    }

    default fn token_decimals(&self) -> u8 {
        self.get().decimals.clone()
    }
}
