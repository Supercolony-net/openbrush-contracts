pub use crate::{
    psp1155::*,
    traits::psp1155::extensions::metadata::*,
};
use brush::declare_storage_trait;
pub use derive::PSP1155MetadataStorage;
use ink_prelude::string::String;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP1155MetadataData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP1155MetadataData {
    pub uri: Option<String>,
}

declare_storage_trait!(PSP1155MetadataStorage, PSP1155MetadataData);

impl<T: PSP1155MetadataStorage> PSP1155Metadata for T {
    default fn uri(&self, _id: Id) -> Option<String> {
        self.get().uri.clone()
    }
}
