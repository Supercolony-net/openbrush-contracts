pub use crate::{
    psp22::*,
    traits::psp22::extensions::metadata::*,
};
use brush::declare_storage_trait;
pub use derive::PSP22MetadataStorage;
use ink_prelude::string::String;
use ink_storage::{
    traits::SpreadLayout,
    Lazy,
};

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

impl<T: PSP22MetadataStorage> PSP22Metadata for T {
    default fn token_name(&self) -> Option<String> {
        Lazy::get(&self.get().name).clone()
    }

    default fn token_symbol(&self) -> Option<String> {
        Lazy::get(&self.get().symbol).clone()
    }

    default fn token_decimals(&self) -> u8 {
        Lazy::get(&self.get().decimals).clone()
    }
}
