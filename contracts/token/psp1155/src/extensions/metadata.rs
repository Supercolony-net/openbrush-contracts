use crate::traits::Id;
/// Metadata for PSP1155
use brush::{
    declare_storage_trait,
    traits::InkStorage,
};
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;
pub use psp1155_derive::PSP1155MetadataStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP1155MetadataData {
    pub uri: Option<String>,
}

declare_storage_trait!(PSP1155MetadataStorage, PSP1155MetadataData);

#[brush::wrapper]
pub type PSP1155MetadataWrapper = dyn PSP1155Metadata;

#[brush::trait_definition]
pub trait PSP1155Metadata: PSP1155MetadataStorage {
    /// Returns the uri for token type of id.
    #[ink(message)]
    fn uri(&self, _id: Id) -> Option<String> {
        self.get().uri.clone()
    }
}
