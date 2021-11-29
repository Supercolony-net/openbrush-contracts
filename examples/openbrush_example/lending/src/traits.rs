use brush::{
    declare_storage_trait,
    traits::InkStorage,
};
use ink_prelude::string::String;
use ink_storage::traits::SpreadLayout;
pub use lending_derive::LendingStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct LendingData {
    pub string: String,
}

declare_storage_trait!(LendingStorage, LendingData);

// we will declare a trait which holds getters and setters for our storage struct
#[brush::trait_definition]
pub trait LendingStorageTrait: LendingStorage {
    #[ink(message)]
    fn string(&self) -> String {
        self.get().string.clone()
    }
}
