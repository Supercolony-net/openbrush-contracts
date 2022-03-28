pub use crate::{
    ownable::*,
    traits::diamond::*,
};
use brush::{
    declare_storage_trait,
    modifiers,
    traits::Hash,
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
    // list of all known facets
    pub facet_code_hashes: Vec<Hash>,
    // mapping of facet to its position in all facets list
    pub hash_position: Mapping<Hash, u16>,
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

impl<T: DiamondStorage> Diamond for T {
    #[modifiers(only_owner)]
    default fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(diamond_cut, init)
    }

    default fn facets(&self) -> Vec<(Hash, Vec<Selector>)> {
        self.get()
            .facet_code_hashes
            .iter()
            .map(|hash| self.get().hash_to_selectors.get(hash).unwrap_or(Vec::<Selector>::new()))
            .collect()
    }

    default fn facet_function_selectors(&self, facet: Hash) -> Vec<Selector> {
        self.get()
            .hash_to_selectors
            .get(facet)
            .unwrap_or(Vec::<Selector>::new())
    }

    default fn facet_code_hashes(&self) -> Vec<Hash> {
        self.get().facet_code_hashes
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

impl<T: DiamondStorage> DiamondInternal for T {
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

        if self.get().hash_to_selectors.get(code_hash).is_none() {
            // register hash
            self.get_mut()
                .hash_to_selectors
                .insert(&code_hash, &Vec::<Selector>::new());
            let hashes = self.get().facet_code_hashes.len();
            self.get_mut().facet_code_hashes.push(code_hash);
            self.get_mut().hash_position.insert(&code_hash, &hashes);
        }

        self.get_mut().selector_to_hash.insert(&selector, &code_hash);
        self.get_mut().hash_to_selectors.get(&code_hash).unwrap().push(selector);
        let selectors = self.get().hash_to_selectors.get(&code_hash).unwrap().len();
        self.get_mut().selector_position.insert(&selector, &(selectors - 1));

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

        self.get().selector_to_hash.remove(selector);
        let selector_pos = self.get().selector_position.get(&selector).unwrap();
        self.get_mut().selector_position.remove(&selector);

        if self.get().hash_to_selectors.get(&code_hash).unwrap().len() == 1 {
            let facet_pos = self.get().hash_position.get(&code_hash).unwrap();
            self.get_mut().hash_position.remove(&code_hash);
            self.get_mut().hash_to_selectors.remove(&code_hash);
            let last_hash = self.get_mut().facet_code_hashes.pop().unwrap();
            if (self.get().facet_code_hashes.len() < facet_pos) {
                self.get_mut().facet_code_hashes[facet_pos] = last_hash;
                self.get_mut().hash_position.insert(&last_hash, &facet_pos);
            }
        } else {
            let last_selector = self.get_mut().hash_to_selectors.get(&code_hash).unwrap().pop().unwrap();
            if (last_selector != selector) {
                let mut vec = self.get().hash_to_selectors.get(&code_hash).unwrap();
                vec[selector_pos] = last_selector;
                self.get_mut().hash_to_selectors.insert(&code_hash, &vec);
                self.get_mut().selector_position.insert(&last_selector, &selector_pos);
            }
        }

        Ok(())
    }
}
