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

pub use crate::{
    diamond,
    diamond::extensions::diamond_loupe,
    ownable,
    traits::{
        diamond::extensions::diamond_loupe::*,
        ownable::*,
    },
};
pub use diamond::Internal as _;
pub use ownable::Internal as _;

use ink::prelude::vec::Vec;
use openbrush::{
    storage::{
        Mapping,
        ValueGuard,
    },
    traits::{
        Hash,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Loupe);

#[derive(Default, Debug, scale::Decode, scale::Encode)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Loupe {
    // number of registered code hashes
    pub code_hashes: u32,
    // mapping of facet to its position in all facets list
    pub hash_to_id: Mapping<Hash, u32>,
    // mapping of facet id to its facet
    pub id_to_hash: Mapping<u32, Hash, ValueGuard<u32>>,
    pub _reserved: Option<()>,
}

impl diamond::DiamondCut for Loupe {
    #[inline(always)]
    fn on_add_facet(&mut self, code_hash: Hash) {
        let hash_id = self.code_hashes;
        self.hash_to_id.insert(&code_hash, &hash_id);
        self.id_to_hash.insert(hash_id, &code_hash);
        self.code_hashes += 1;
    }

    fn on_remove_facet(&mut self, code_hash: Hash) {
        let new_hash_id = self.code_hashes - 1;
        let removed_hash_id = self.hash_to_id.get(&code_hash).unwrap();
        let last_hash = self.id_to_hash.get(new_hash_id).unwrap();

        if last_hash != code_hash {
            self.id_to_hash.insert(removed_hash_id, &last_hash);
            self.hash_to_id.insert(&last_hash, &removed_hash_id);
            self.id_to_hash.remove(new_hash_id);
        } else {
            self.id_to_hash.remove(removed_hash_id);
        }

        self.hash_to_id.remove(&code_hash);
        self.code_hashes = new_hash_id;
    }
}

impl<T: Storage<diamond::Data<Loupe>>> DiamondLoupe for T {
    default fn facets(&self) -> Vec<FacetCut> {
        let mut out_vec = Vec::new();
        let data = &self.data().handler;
        for i in 0..data.code_hashes {
            let hash = data.id_to_hash.get(i).unwrap();
            let selectors = self.data().hash_to_selectors.get(&hash).unwrap();
            out_vec.push(FacetCut { hash, selectors })
        }
        out_vec
    }

    default fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
        self.data()
            .hash_to_selectors
            .get(&facet)
            .unwrap_or(Vec::<Selector>::new())
    }

    default fn facet_code_hashes(&self) -> Vec<Hash> {
        let mut out_vec = Vec::new();
        for i in 0..self.data().handler.code_hashes {
            out_vec.push(self.data().handler.id_to_hash.get(i).unwrap())
        }
        out_vec
    }

    default fn facet_code_hash(&self, selector: Selector) -> Option<Hash> {
        self.data().selector_to_hash.get(&selector)
    }
}
