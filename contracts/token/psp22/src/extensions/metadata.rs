use brush::{
    declare_storage_trait,
    traits::InkStorage,
};
use ink_prelude::string::String;
use ink_storage::{
    traits::SpreadLayout,
    Lazy,
};
pub use psp22_derive::PSP22MetadataStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22MetadataData {
    pub name: Lazy<Option<String>>,
    pub symbol: Lazy<Option<String>>,
    pub decimals: Lazy<u8>,
}

declare_storage_trait!(PSP22MetadataStorage, PSP22MetadataData);

/// Trait that contains metadata
#[brush::trait_definition]
pub trait PSP22Metadata: PSP22MetadataStorage {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String> {
        Lazy::get(&self.get().name).clone()
    }

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String> {
        Lazy::get(&self.get().symbol).clone()
    }

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8 {
        Lazy::get(&self.get().decimals).clone()
    }
}
