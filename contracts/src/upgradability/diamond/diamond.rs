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
    ownable,
    traits::{
        diamond::*,
        ownable::*,
    },
};
pub use ownable::Internal as _;

use ink_env::{
    call::{
        DelegateCall,
        ExecutionInput,
        Selector as InkSelector,
    },
    Clear,
};
use ink_prelude::vec::Vec;
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
};
use openbrush::{
    modifiers,
    storage::Mapping,
    traits::{
        Hash,
        OccupiedStorage,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

// TODO: Add support of Erc165
// TODO: Refactor to embed loupe via generic
#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct Data<D: DiamondCut = ()> {
    // Selector mapped to its facet
    pub selector_to_hash: Mapping<Selector, Hash>,
    // Facet mapped to all functions it supports
    pub hash_to_selectors: Mapping<Hash, Vec<Selector>>,
    // Handler of each facet add and remove.
    // It is empty by default but can be extended with loup logic.
    pub handler: D,
}

impl<D, T> Diamond for T
where
    D: DiamondCut,
    T: Storage<ownable::Data>,
    T: Storage<Data<D>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<D>>,
{
    #[modifiers(ownable::only_owner)]
    default fn diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(diamond_cut, init)
    }
}

pub trait Internal {
    fn _emit_diamond_cut_event(&self, diamond_cut: &Vec<FacetCut>, init: &Option<InitCall>);

    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;

    fn _remove_facet(&mut self, code_hash: Hash);

    fn _remove_selectors(&mut self, facet_cut: &FacetCut);
}

impl<D, T> Internal for T
where
    D: DiamondCut,
    T: Storage<Data<D>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<D>>,
{
    default fn _emit_diamond_cut_event(&self, _diamond_cut: &Vec<FacetCut>, _init: &Option<InitCall>) {}

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
                    let selector_hash = self.data().selector_to_hash.get(&selector);

                    if selector_hash.and_then(|hash| Some(hash == code_hash)).unwrap_or(false) {
                        // selector already registered to this hash -> no action
                        continue
                    } else if selector_hash.is_some() {
                        // selector already registered to another hash -> error
                        return Err(DiamondError::ReplaceExisting(selector_hash.unwrap()))
                    } else {
                        // map selector to its facet
                        self.data().selector_to_hash.insert(&selector, &code_hash);
                    }
                }

                if self.data().hash_to_selectors.get(&code_hash).is_none() {
                    self.data().handler.on_add_facet(code_hash);
                }
                // remove selectors from this facet which may be registered but will not be used anymore
                self._remove_selectors(facet_cut);
                // map this code hash to its selectors
                self.data().hash_to_selectors.insert(&code_hash, &facet_cut.selectors);
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

        let delegate_code = self.data().selector_to_hash.get(&selector);

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
        let vec = self.data().hash_to_selectors.get(&code_hash).unwrap();
        vec.iter().for_each(|old_selector| {
            self.data().selector_to_hash.remove(&old_selector);
        });
        self.data().hash_to_selectors.remove(&code_hash);
        self.data().handler.on_remove_facet(code_hash);
    }

    default fn _remove_selectors(&mut self, facet_cut: &FacetCut) {
        let selectors = self
            .data()
            .hash_to_selectors
            .get(&facet_cut.hash)
            .unwrap_or(Vec::<Selector>::new());
        for selector in selectors.iter() {
            if !facet_cut.selectors.contains(&selector) {
                self.data().selector_to_hash.remove(&selector);
            }
        }
    }
}

pub trait DiamondCut: SpreadLayout + SpreadAllocate {
    fn on_add_facet(&mut self, code_hash: Hash);

    fn on_remove_facet(&mut self, code_hash: Hash);
}

impl DiamondCut for () {
    #[inline(always)]
    fn on_add_facet(&mut self, _code_hash: Hash) {}

    #[inline(always)]
    fn on_remove_facet(&mut self, _code_hash: Hash) {}
}
