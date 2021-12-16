/// Metadata for PSP1155
pub use crate::traits::psp1155::Id;
use ink_prelude::string::String;

#[brush::wrapper]
pub type PSP1155MetadataRef = dyn PSP1155Metadata;

#[brush::trait_definition]
pub trait PSP1155Metadata {
    /// Returns the uri for token type of id.
    #[ink(message)]
    fn uri(&self, _id: Id) -> Option<String>;
}
