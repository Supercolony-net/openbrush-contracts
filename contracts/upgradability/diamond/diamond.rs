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

impl<T: DiamondStorage + Flush + DiamondCut> Diamond for T {
    #[modifiers(only_owner)]
    default fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(diamond_cut, init)
    }
}

pub trait DiamondInternal {
    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;

    fn _handle_replace_immutable(&mut self, hash: Hash) -> Result<(), DiamondError>;

    fn _handle_replace_existing(&mut self, facet_cut: &FacetCut) -> Result<(), DiamondError>;

    fn _handle_existing_selector(&mut self, selector: Selector);

    fn _add_function(&mut self, code_hash: Hash, selector: Selector);

    fn _emit_diamond_cut_event(&self, diamond_cut: &Vec<FacetCut>, init: &Option<InitCall>);
}

impl<T: DiamondStorage + Flush + DiamondCut> DiamondInternal for T {
    default fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        for facet_cut in diamond_cut.iter() {
            let code_hash = facet_cut.hash;
            self._handle_replace_immutable(code_hash)?;
            self._handle_replace_existing(&facet_cut)?;
            for selector in facet_cut.selectors.iter() {
                self._handle_existing_selector(*selector);
                self._add_function(code_hash, *selector);
            }
        }

        self._emit_diamond_cut_event(&diamond_cut, &init);

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

    default fn _init_call(&self, call: InitCall) -> ! {
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

    default fn _handle_replace_immutable(&mut self, hash: Hash) -> Result<(), DiamondError> {
        return if hash == self.get().self_hash {
            Err(DiamondError::ImmutableFunction)
        } else {
            Ok(())
        }
    }

    default fn _handle_replace_existing(&mut self, facet_cut: &FacetCut) -> Result<(), DiamondError> {
        for selector in facet_cut.selectors.iter() {
            if self
                .get()
                .selector_to_hash
                .get(&selector)
                .and_then(|hash| {
                    if hash == facet_cut.hash {
                        return Some(hash)
                    };
                    None
                })
                .is_some()
            {
                return Err(DiamondError::ReplaceExisting)
            };
        }
        Ok(())
    }

    default fn _handle_existing_selector(&mut self, selector: Selector) {
        // if this selector exists it means we are replacing the facet with new facet and have to
        // delete old facet, as some functions may have been removed
        self.get().selector_to_hash.get(selector).and_then(|hash| {
            let vec = self.get().hash_to_selectors.get(&hash).unwrap();
            vec.iter().for_each(|old_selector| {
                self.get_mut().selector_to_hash.remove(&old_selector);
            });
            self.get_mut().hash_to_selectors.remove(&hash);
            self._on_remove_facet(hash);
            Some(hash)
        });
    }

    default fn _add_function(&mut self, code_hash: Hash, selector: Selector) {
        let mut vec = self.get().hash_to_selectors.get(&code_hash).unwrap_or_else(|| {
            self._on_add_function(code_hash);
            Vec::<Selector>::new()
        });

        vec.push(selector);

        self.get_mut().selector_to_hash.insert(&selector, &code_hash);
        self.get_mut().hash_to_selectors.insert(&code_hash, &vec);
    }

    fn _emit_diamond_cut_event(&self, _diamond_cut: &Vec<FacetCut>, _init: &Option<InitCall>) {}
}

pub trait DiamondCut {
    fn _on_add_function(&mut self, code_hash: Hash);

    fn _on_remove_facet(&mut self, code_hash: Hash);
}

impl<T> DiamondCut for T {
    default fn _on_add_function(&mut self, _code_hash: Hash) {}

    default fn _on_remove_facet(&mut self, _code_hash: Hash) {}
}
