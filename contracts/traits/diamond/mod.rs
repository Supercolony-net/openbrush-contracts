pub use crate::traits::{
    errors::{
        DiamondError,
        OwnableError,
    },
    ownable::*,
};
use brush::traits::Hash;
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type DiamondRef = dyn Diamond;

pub type Selector = [u8; 4];

/// Struct which we use to initialize/update/remove a facet in the diamond
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FacetCut {
    /// The `hash` of the code that should be executed.
    pub hash: Hash,
    /// The selector bytes that identify the function that should be called.
    pub selectors: Vec<[u8; 4]>,
}

/// Struct which we use to initialize the diamond contract
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct InitCall {
    /// The `hash` of the code that should be executed.
    pub hash: Hash,
    /// The selector bytes that identify the function that should be called.
    pub selector: Selector,
    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,
}

/// Trait to be implemented in the contract which holds the diamond storage
/// Also implements functions of Diamond Loupe to lookup the functionality of the diamond contract
#[brush::trait_definition]
pub trait Diamond {
    /// This function is used to add, replace and remove facets from the diamond
    ///
    /// `cuts` vector of facet cuts to be mutated, each cut contains the code hash of the facet
    /// as well as the selectors of functions along with the action to be performed with the
    /// correspondent selector (see enum `FacetCutAction`)
    /// `init` optional struct which identifies a call to be executed, this struct contains the code hash
    /// of the executed contract, selector of the executed function and input data to be passed to the called
    #[ink(message)]
    fn diamond_cut(&mut self, cuts: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    /// Returns code hashes of all registered facets along with their registered function selectors
    #[ink(message)]
    fn facets(&self) -> Vec<(Hash, Vec<Selector>)>;

    /// Returns all the function selectors supported by a specific facet
    #[ink(message)]
    fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector>;

    /// Returns all the code hashes of facets used by the diamond
    #[ink(message)]
    fn facet_code_hashes(&self) -> Vec<Hash>;

    /// Returns the code hash of a facet which supports the given selector
    /// Returns empty hash if selector is not found
    #[ink(message)]
    fn facet_code_hash(&self, selector: Selector) -> Option<Hash>;
}
