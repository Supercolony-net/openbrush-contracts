/// Metadata for PSP721
use brush::{
    declare_storage_trait,
    traits::InkStorage,
};
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;
pub use psp721_derive::PSP721MetadataStorage;

use crate::traits::Id;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP721MetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
}

declare_storage_trait!(PSP721MetadataStorage, PSP721MetadataData);

#[brush::wrapper]
pub type PSP721MetadataRef = dyn PSP721Metadata;

#[brush::trait_definition]
pub trait PSP721Metadata: PSP721MetadataStorage {
    /// Returns the token name.
    #[ink(message)]
    fn name(&self) -> Option<String> {
        self.get().name.clone()
    }

    /// Returns the token symbol.
    #[ink(message)]
    fn symbol(&self) -> Option<String> {
        self.get().symbol.clone()
    }

    /// Returns the Uniform Resource Identifier (URI) for `id` token.
    #[ink(message)]
    fn uri(&self, _id: Id) -> Option<String> {
        None
    }

    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>) {
        self.get_mut().name = name;
        self.get_mut().symbol = symbol;
    }
}
