// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::traits::diamond::*;
use ink_prelude::vec::Vec;
use openbrush::traits::Hash;

#[openbrush::wrapper]
pub type DiamondLoupeRef = dyn DiamondLoupe;

/// Trait which implements functions of Diamond Loupe to lookup the functionality of the diamond contract
#[openbrush::trait_definition]
pub trait DiamondLoupe {
    /// Returns code hashes of all registered facets along with their registered function selectors
    #[ink(message)]
    fn facets(&self) -> Vec<FacetCut>;

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
