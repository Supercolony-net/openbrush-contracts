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
    ownable::*,
    traits::diamond::*,
};
use ink_env::{
    call::{
        DelegateCall,
        ExecutionInput,
        Selector as InkSelector,
    },
    Clear,
};
use ink_prelude::vec::Vec;
use openbrush::{
    modifiers,
    traits::{
        Flush,
        Hash,
    },
};

pub use derive::DiamondStorage;
use openbrush::storage::Mapping;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::DiamondData");

// TODO: Add support of Erc165
#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct DiamondData {
    pub ownable: OwnableData,
    // selector mapped to its facet
    pub selector_to_hash: Mapping<Selector, Hash>,
    // facet mapped to all functions it supports
    pub hash_to_selectors: Mapping<Hash, Vec<Selector>>,
}

pub trait DiamondStorage: OwnableStorage<Data = OwnableData> + ::openbrush::traits::InkStorage {
    type Data;
    fn get(&self) -> &<Self as DiamondStorage>::Data;
    fn get_mut(&mut self) -> &mut <Self as DiamondStorage>::Data;
}

#[cfg(not(feature = "proxy"))]
impl<T: DiamondStorage<Data = DiamondData>> OwnableStorage for T {
    type Data = OwnableData;

    fn get(&self) -> &Self::Data {
        &DiamondStorage::get(self).ownable
    }

    fn get_mut(&mut self) -> &mut Self::Data {
        &mut DiamondStorage::get_mut(self).ownable
    }
}

impl<T: DiamondStorage<Data = DiamondData> + Flush + DiamondCut> Diamond for T {
    #[modifiers(only_owner)]
    default fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(diamond_cut, init)
    }
}

pub trait DiamondInternal {
    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;

    fn _remove_facet(&mut self, code_hash: Hash);

    fn _remove_selectors(&mut self, facet_cut: &FacetCut);

    fn _emit_diamond_cut_event(&self, diamond_cut: &Vec<FacetCut>, init: &Option<InitCall>);
}

impl<T: DiamondStorage<Data = DiamondData> + Flush + DiamondCut> DiamondInternal for T {
    default fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        for facet_cut in diamond_cut.iter() {
            let code_hash = facet_cut.hash;
            if code_hash.is_clear() {
                return Err(DiamondError::EmptyCodeHash)
            }
            if facet_cut.selectors.is_empty() {
                // means that we want to remove this facet
                self._remove_facet(code_hash);
            } else {
                for selector in facet_cut.selectors.iter() {
                    let selector_hash = DiamondStorage::get(self).selector_to_hash.get(&selector);

                    if selector_hash.and_then(|hash| Some(hash == code_hash)).unwrap_or(false) {
                        // selector already registered to this hash -> no action
                        continue
                    } else if selector_hash.is_some() {
                        // selector already registered to another hash -> error
                        return Err(DiamondError::ReplaceExisting(selector_hash.unwrap()))
                    } else {
                        // map selector to its facet
                        DiamondStorage::get_mut(self)
                            .selector_to_hash
                            .insert(&selector, &code_hash);
                    }
                }

                if DiamondStorage::get(self).hash_to_selectors.get(&code_hash).is_none() {
                    self._on_add_facet(code_hash);
                }
                // remove selectors from this facet which may be registered but will not be used anymore
                self._remove_selectors(facet_cut);
                // map this code hash to its selectors
                DiamondStorage::get_mut(self)
                    .hash_to_selectors
                    .insert(&code_hash, &facet_cut.selectors);
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

        let delegate_code = DiamondStorage::get(self).selector_to_hash.get(&selector);

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

    default fn _remove_facet(&mut self, code_hash: Hash) {
        let vec = DiamondStorage::get(self).hash_to_selectors.get(&code_hash).unwrap();
        vec.iter().for_each(|old_selector| {
            DiamondStorage::get_mut(self).selector_to_hash.remove(&old_selector);
        });
        DiamondStorage::get_mut(self).hash_to_selectors.remove(&code_hash);
        self._on_remove_facet(code_hash);
    }

    default fn _remove_selectors(&mut self, facet_cut: &FacetCut) {
        let selectors = DiamondStorage::get(self)
            .hash_to_selectors
            .get(&facet_cut.hash)
            .unwrap_or(Vec::<Selector>::new());
        for selector in selectors.iter() {
            if !facet_cut.selectors.contains(&selector) {
                DiamondStorage::get_mut(self).selector_to_hash.remove(&selector);
            }
        }
    }

    default fn _emit_diamond_cut_event(&self, _diamond_cut: &Vec<FacetCut>, _init: &Option<InitCall>) {}
}

pub trait DiamondCut {
    fn _on_add_facet(&mut self, code_hash: Hash);

    fn _on_remove_facet(&mut self, code_hash: Hash);
}

impl<T> DiamondCut for T {
    default fn _on_add_facet(&mut self, _code_hash: Hash) {}

    default fn _on_remove_facet(&mut self, _code_hash: Hash) {}
}
