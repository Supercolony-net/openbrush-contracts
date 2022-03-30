pub use crate::{
    ownable::*,
    traits::diamond::*,
};
use brush::{
    declare_storage_trait,
    modifiers,
    traits::{
        Flush,
        Hash,
    },
};
use ink_env::call::{
    DelegateCall,
    ExecutionInput,
    Selector as InkSelector,
};
use ink_prelude::vec::Vec;
use ink_storage::Mapping;

pub use derive::DiamondStorage;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::DiamondData");

// TODO: Add support of Erc165
#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct DiamondData {
    pub ownable: OwnableData,
    // selector mapped to its facet
    pub selector_to_hash: Mapping<Selector, Hash>,
    // facet mapped to all functions it supports
    pub hash_to_selectors: Mapping<Hash, Vec<Selector>>,
    // number of registered code hashes
    pub code_hashes: u16,
    // mapping of facet to its position in all facets list
    pub hash_to_id: Mapping<Hash, u16>,
    // mapping of facet id to its facet
    pub id_to_hash: Mapping<u16, Hash>,
    // mapping of selector to its position in facet selectors list
    pub selector_position: Mapping<Selector, u16>,
    // code hash of diamond contract for immutable functions
    pub self_hash: Hash,
}

declare_storage_trait!(DiamondStorage, DiamondData);

impl<T: DiamondStorage> OwnableStorage for T {
    fn get(&self) -> &OwnableData {
        &DiamondStorage::get(self).ownable
    }

    fn get_mut(&mut self) -> &mut OwnableData {
        &mut DiamondStorage::get_mut(self).ownable
    }
}

impl<T: DiamondStorage + Flush> Diamond for T {
    #[modifiers(only_owner)]
    default fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(diamond_cut, init)
    }

    default fn facets(&self) -> Vec<(Hash, Vec<Selector>)> {
        let mut out_vec = Vec::new();
        for i in 0..self.get().code_hashes {
            let hash = self.get().id_to_hash.get(&i).unwrap();
            out_vec.push((hash, self.get().hash_to_selectors.get(&hash).unwrap()))
        }
        out_vec
    }

    default fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
        self.get()
            .hash_to_selectors
            .get(facet)
            .unwrap_or(Vec::<Selector>::new())
    }

    default fn facet_code_hashes(&self) -> Vec<Hash> {
        let mut out_vec = Vec::new();
        for i in 0..self.get().code_hashes {
            out_vec.push(self.get().id_to_hash.get(&i).unwrap())
        }
        out_vec
    }

    default fn facet_code_hash(&self, selector: Selector) -> Hash {
        self.get().selector_to_hash.get(selector).unwrap_or(Default::default())
    }
}

pub trait DiamondInternal {
    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;

    fn _add_function(&mut self, code_hash: Hash, selector: Selector) -> Result<(), DiamondError>;

    fn _replace_function(&mut self, code_hash: Hash, selector: Selector) -> Result<(), DiamondError>;

    fn _remove_function(&mut self, selector: Selector) -> Result<(), DiamondError>;
}

impl<T: DiamondStorage + Flush> DiamondInternal for T {
    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        for facet_cut in diamond_cut.iter() {
            let code_hash = facet_cut.hash;
            for selector in facet_cut.selectors.iter() {
                let action: FacetCutAction = selector.1.into();
                match action {
                    FacetCutAction::Add => self._add_function(code_hash, selector.0),
                    FacetCutAction::Replace => self._replace_function(code_hash, selector.0),
                    FacetCutAction::Remove => self._remove_function(selector.0),
                    FacetCutAction::Unknown => Err(DiamondError::IncorrectFacetCutAction),
                }?;
            }
        }

        if init.is_some() {
            self.flush();
            self._init_call(init.unwrap());
        }

        Ok(())
    }

    default fn _fallback(&self) -> ! {
        let selector = ink_env::decode_input::<Selector>().unwrap_or_else(|_| panic!("Calldata error"));

        let delegate_code = self.get().selector_to_hash.get(selector);

        if delegate_code.is_none() {
            panic!("Function is not registered");
        }

        ink_env::call::build_call::<ink_env::DefaultEnvironment>()
            .call_type(DelegateCall::new().code_hash(delegate_code.unwrap()))
            .call_flags(
                ink_env::CallFlags::default()
                // We don't plan to use the input data after the delegated call, so the 
                // input data can be forwarded to delegated contract to reduce the gas usage.
                .set_forward_input(true)
                // We don't plan to return back to that contract after execution, so we 
                // marked delegated call as "tail", to end the execution of the contract.
                .set_tail_call(true),
            )
            .fire()
            .unwrap_or_else(|err| panic!("delegate call to {:?} failed due to {:?}", delegate_code, err));
        unreachable!("the _fallback call will never return since `tail_call` was set");
    }

    fn _init_call(&self, call: InitCall) -> ! {
        ink_env::call::build_call::<ink_env::DefaultEnvironment>()
            .call_type(DelegateCall::new().code_hash(call.hash))
            .exec_input(ExecutionInput::new(InkSelector::new(call.selector)).push_arg(call.input))
            .call_flags(ink_env::CallFlags::default()
            // We don't plan to return back to that contract after execution, so we
            // marked delegated call as "tail", to end the execution of the contract.
            .set_tail_call(true))
            .returns::<()>()
            .fire()
            .unwrap_or_else(|err| panic!("init call failed due to {:?}", err));
        unreachable!("the _init_call call will never return since `tail_call` was set");
    }

    fn _add_function(&mut self, code_hash: Hash, selector: Selector) -> Result<(), DiamondError> {
        if self.get().selector_to_hash.get(selector).is_some() {
            return Err(DiamondError::FunctionAlreadyExists)
        }

        let mut vec = self.get().hash_to_selectors.get(&code_hash).unwrap_or_else(|| {
            let hash_id = self.get().code_hashes;
            self.get_mut().hash_to_id.insert(&code_hash, &hash_id);
            self.get_mut().id_to_hash.insert(&hash_id, &code_hash);
            self.get_mut().code_hashes += 1;
            Vec::<Selector>::new()
        });

        vec.push(selector);

        self.get_mut().selector_to_hash.insert(&selector, &code_hash);
        self.get_mut().hash_to_selectors.insert(&code_hash, &vec);
        self.get_mut()
            .selector_position
            .insert(&selector, &(vec.len() as u16 - 1));

        Ok(())
    }

    fn _replace_function(&mut self, code_hash: Hash, selector: Selector) -> Result<(), DiamondError> {
        if self
            .get()
            .selector_to_hash
            .get(selector)
            .ok_or(DiamondError::FunctionDoesNotExist)?
            == code_hash
        {
            return Err(DiamondError::ReplaceExisting)
        }

        self._remove_function(selector)?;
        self._add_function(code_hash, selector)
    }

    fn _remove_function(&mut self, selector: Selector) -> Result<(), DiamondError> {
        let code_hash = self
            .get()
            .selector_to_hash
            .get(selector)
            .ok_or(DiamondError::FunctionDoesNotExist)?;

        if code_hash == self.get().self_hash {
            return Err(DiamondError::ImmutableFunction)
        }

        let selector_pos = self.get().selector_position.get(&selector).unwrap();
        let mut selector_vec = self.get().hash_to_selectors.get(&code_hash).unwrap();
        let last_selector = selector_vec.pop().unwrap();

        // if the popped selector is not the one we are removing we will put it on
        // on the place of the removed vector
        if last_selector != selector {
            selector_vec[selector_pos as usize] = last_selector;
            self.get_mut().selector_position.insert(&last_selector, &selector_pos);
        }

        // if the vector of selectors is empty we can remove the hash
        if selector_vec.is_empty() {
            // we get id of our hash
            let hash_id = self.get().hash_to_id.get(&code_hash).unwrap();
            // we will decrease the count of hashes
            let last_id = self.get().code_hashes - 1;
            self.get_mut().code_hashes = last_id;
            // we are removing our hash so no need to track its id
            self.get_mut().hash_to_id.remove(&code_hash);
            // if removed hash was not the last added hash we need to change ids
            if hash_id != last_id {
                // current number of hashes is the id of hash
                let last_hash = self.get().id_to_hash.get(&last_id).unwrap();
                // change the id of last hash to id of currently removed hash
                self.get_mut().id_to_hash.insert(&hash_id, &last_hash);
            } else {
                // else we just remove the id
                self.get_mut().id_to_hash.remove(&hash_id);
            }
            // remove the vector of selectors
            self.get_mut().hash_to_selectors.remove(&code_hash);
        } else {
            // if the vector of selectors is not empty we did not remove it and
            // we need to write the updated vec into storage
            self.get_mut().hash_to_selectors.insert(&code_hash, &selector_vec);
        }

        self.get_mut().selector_to_hash.remove(&selector);
        self.get_mut().selector_position.remove(&selector);

        Ok(())
    }
}
