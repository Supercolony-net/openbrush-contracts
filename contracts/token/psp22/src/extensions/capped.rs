/// Extension of [`PSP22`] which supports token wrapping
use crate::traits::*;

use brush::{
    declare_storage_trait,
    traits::{
        Balance,
        InkStorage,
    },
};
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;
pub use psp22_derive::PSP22CappedStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22CappedData {
    pub cap: Balance,
}

declare_storage_trait!(PSP22CappedStorage, PSP22CappedData);

#[brush::wrapper]
pub type PSP22CappedWrapper = dyn PSP22Capped + PSP22;

#[brush::trait_definition]
pub trait PSP22Capped: PSP22CappedStorage + PSP22 {
    #[ink(message)]
    fn cap(&self) -> Balance {
        PSP22CappedStorage::get(self).cap
    }

    fn init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
        if cap <= 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
        }
        PSP22CappedStorage::get_mut(self).cap = cap;
        Ok(())
    }
}
