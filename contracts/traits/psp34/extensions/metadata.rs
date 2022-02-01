/// Metadata for PSP34
pub use crate::traits::errors::PSP34Error;
pub use crate::traits::psp34::Id;
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type PSP34MetadataRef = dyn PSP34Metadata;

#[brush::trait_definition]
pub trait PSP34Metadata {
    /// Returns the attribute of `id` for the given `key`.
    ///
    /// If `id` is a collection id of the token, it returns attributes for collection.
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}
