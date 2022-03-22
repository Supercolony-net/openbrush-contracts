use crate::diamond::Selector;
pub use crate::traits::{
    errors::{
        DiamondError,
        OwnableError,
    },
    ownable::*,
};
use brush::traits::Hash;

#[brush::wrapper]
pub type DiamondRef = dyn Diamond;

pub enum FacetCutAction {
    Add,
    Replace,
    Remove,
}

// TODO: Comment
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FacetCut {
    /// The `hash` of the code that should be executed.
    pub hash: Hash,
    /// The selector bytes that identifies the function that should be called.
    pub selectors: Vec<([u8; 4], FacetCutAction)>,
}

// TODO: Comment
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InitCall {
    /// The `hash` of the code that should be executed.
    pub hash: Hash,
    /// The selector bytes that identifies the function that should be called.
    pub selector: Selector,
    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,
}

// TODO: Comment
#[brush::trait_definition]
pub trait Diamond {
    // TODO: Comment
    #[ink(message)]
    fn diamond_cut(&mut self, facets: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;
}
