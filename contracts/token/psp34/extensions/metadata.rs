pub use crate::{
    psp34::*,
    traits::psp34::extensions::metadata::*,
};
use brush::declare_storage_trait;
pub use derive::PSP34MetadataStorage;
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP34MetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
}

declare_storage_trait!(PSP34MetadataStorage, PSP34MetadataData);

impl<T: PSP34MetadataStorage> PSP34Metadata for T {
    default fn name(&self) -> Option<String> {
        self.get().name.clone()
    }

    default fn symbol(&self) -> Option<String> {
        self.get().symbol.clone()
    }

    default fn uri(&self, _id: Id) -> Option<String> {
        None
    }
}

pub trait PSP34MetadataInternal {
    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>);
}

impl<T: PSP34MetadataStorage> PSP34MetadataInternal for T {
    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>) {
        self.get_mut().name = name;
        self.get_mut().symbol = symbol;
    }
}
