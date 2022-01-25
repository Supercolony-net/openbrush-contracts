/// Metadata for PSP34
pub use crate::traits::errors::PSP34Error;
pub use crate::traits::psp34::Id;
use ink_prelude::string::String;

#[brush::wrapper]
pub type PSP34MetadataRef = dyn PSP34Metadata;

#[brush::trait_definition]
pub trait PSP34Metadata {
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
