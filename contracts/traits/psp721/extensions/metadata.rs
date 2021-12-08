/// Metadata for PSP721
pub use crate::traits::errors::PSP721Error;
pub use crate::traits::psp721::Id;
use ink_prelude::string::String;

#[brush::wrapper]
pub type PSP721MetadataRef = dyn PSP721Metadata;

#[brush::trait_definition]
pub trait PSP721Metadata {
    /// Returns the token name.
    #[ink(message)]
    fn name(&self) -> Option<String>;

    /// Returns the token symbol.
    #[ink(message)]
    fn symbol(&self) -> Option<String>;

    /// Returns the Uniform Resource Identifier (URI) for `id` token.
    #[ink(message)]
    fn uri(&self, id: Id) -> Option<String>;
}
