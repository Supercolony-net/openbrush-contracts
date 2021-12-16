use ink_prelude::string::String;

#[brush::wrapper]
pub type PSP22MetadataRef = dyn PSP22Metadata;

/// Trait that contains metadata
#[brush::trait_definition]
pub trait PSP22Metadata {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String>;

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String>;

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8;
}
