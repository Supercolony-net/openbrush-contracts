use crate::traits::diamond::Selector;
pub use crate::{
    diamond::*,
    traits::diamond::extensions::diamond_loupe::*,
};
use brush::{
    declare_storage_trait,
    traits::Hash,
};
use ink_prelude::vec::Vec;
use ink_storage::Mapping;

pub use derive::DiamondLoupeStorage;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::DiamondLoupeData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct DiamondLoupeData {
    // number of registered code hashes
    pub code_hashes: u16,
    // mapping of facet to its position in all facets list
    pub hash_to_id: Mapping<Hash, u16>,
    // mapping of facet id to its facet
    pub id_to_hash: Mapping<u16, Hash>,
    pub _reserved: Option<()>,
}

declare_storage_trait!(DiamondLoupeStorage, DiamondLoupeData);

// pub trait DiamondLoupeStorage: DiamondStorage + ::brush::traits::InkStorage {
//     fn get(&self) -> &DiamondLoupeData;
//     fn get_mut(&mut self) -> &mut DiamondLoupeData;
// }

impl<T: DiamondLoupeStorage> DiamondCut for T {
    default fn _on_add_function(&mut self, code_hash: Hash) {
        let hash_id = self.get().code_hashes;
        self.get_mut().hash_to_id.insert(&code_hash, &hash_id);
        self.get_mut().id_to_hash.insert(&hash_id, &code_hash);
        self.get_mut().code_hashes += 1;
    }

    default fn _on_remove_facet(&mut self, code_hash: Hash) {
        let removed_hash_id = self.get().hash_to_id.get(&code_hash).unwrap();
        let last_hash = self.get().id_to_hash.get(&self.get().code_hashes).unwrap();

        if last_hash != code_hash {
            self.get_mut().id_to_hash.insert(&removed_hash_id, &last_hash);
        } else {
            self.get_mut().id_to_hash.remove(&removed_hash_id);
        }

        self.get_mut().hash_to_id.remove(&code_hash);
        self.get_mut().code_hashes -= 1;
    }
}

impl<T: DiamondLoupeStorage + DiamondStorage> DiamondLoupe for T {
    default fn facets(&self) -> Vec<(Hash, Vec<Selector>)> {
        let mut out_vec = Vec::new();
        for i in 0..DiamondLoupeStorage::get(self).code_hashes {
            let hash = DiamondLoupeStorage::get(self).id_to_hash.get(&i).unwrap();
            out_vec.push((hash, DiamondStorage::get(self).hash_to_selectors.get(&hash).unwrap()))
        }
        out_vec
    }

    default fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
        DiamondStorage::get(self)
            .hash_to_selectors
            .get(facet)
            .unwrap_or(Vec::<Selector>::new())
    }

    default fn facet_code_hashes(&self) -> Vec<Hash> {
        let mut out_vec = Vec::new();
        for i in 0..DiamondLoupeStorage::get(self).code_hashes {
            out_vec.push(DiamondLoupeStorage::get(self).id_to_hash.get(&i).unwrap())
        }
        out_vec
    }

    default fn facet_code_hash(&self, selector: Selector) -> Option<Hash> {
        DiamondStorage::get(self).selector_to_hash.get(selector)
    }
}
