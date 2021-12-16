pub use crate::{
    psp1155::*,
    traits::psp1155::extensions::metadata::*,
};
use brush::declare_storage_trait;
pub use derive::PSP1155MetadataStorage;
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP1155MetadataData {
    pub uri: Option<String>,
}

declare_storage_trait!(PSP1155MetadataStorage, PSP1155MetadataData);

impl<T: PSP1155MetadataStorage> PSP1155Metadata for T {
    default fn uri(&self, _id: Id) -> Option<String> {
        self.get().uri.clone()
    }
}
