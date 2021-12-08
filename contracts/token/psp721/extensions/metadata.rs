pub use crate::{
    psp721::*,
    traits::psp721::extensions::metadata::*,
};
use brush::{
    declare_storage_trait,
    traits::InkStorage,
};
pub use derive::PSP721MetadataStorage;
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP721MetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
}

declare_storage_trait!(PSP721MetadataStorage, PSP721MetadataData);

impl<T: PSP721MetadataStorage> PSP721Metadata for T {
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

pub trait PSP721MetadataInternal {
    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>);
}

impl<T: PSP721MetadataStorage> PSP721MetadataInternal for T {
    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>) {
        self.get_mut().name = name;
        self.get_mut().symbol = symbol;
    }
}
