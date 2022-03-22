pub use crate::{
    ownable::*,
    traits::diamond::*,
};
use brush::{
    declare_storage_trait,
    modifiers,
    traits::Hash,
};
use ink_env::call::{DelegateCall, ExecutionInput, Selector as InkSelector};

pub use derive::ProxyStorage;
use ink_storage::Mapping;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::DiamondData");

pub type Selector = [u8; 4];

// TODO: Add support of Erc165
#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct DiamondData {
    pub ownable: OwnableData,
    pub selector_to_hash: Mapping<Selector, Hash>,
    // TODO: Optimize to use Lazy for vector
    pub selectors: Vec<Selector>,
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
    default fn diamond_cut(&mut self, facets: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        self._diamond_cut(facets, init)
    }
}

pub trait DiamondInternal {
    fn _diamond_cut(&mut self, facets: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;
}

impl<T: DiamondStorage> DiamondInternal for T {
    fn _diamond_cut(&mut self, facets: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError> {
        todo!()
    }

    default fn _fallback(&self) -> ! {
        let delegate_code = ...;
        ink_env::call::build_call::<ink_env::DefaultEnvironment>()
            .call_type(DelegateCall::new().code_hash(delegate_code))
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
            .unwrap_or_else(|err| {
                panic!(
                    "delegate call to {:?} failed due to {:?}",
                    delegate_code,
                    err
                )
            });
        unreachable!("the _fallback call will never return since `tail_call` was set");
    }

    fn _init_call(&self, call: InitCall) -> ! {
        ink_env::call::build_call::<ink_env::DefaultEnvironment>()
            .call_type(DelegateCall::new().code_hash(call.hash))
            .exec_input(
                ExecutionInput::new(
                    InkSelector::new(call.selector)
                )
                .push_arg(call.input)
            )
            .call_flags(
                ink_env::CallFlags::default()
                    // We don't plan to return back to that contract after execution, so we
                    // marked delegated call as "tail", to end the execution of the contract.
                    .set_tail_call(true),
            )
            .fire()
            .unwrap_or_else(|err| {
                panic!(
                    "init call failed due to {:?}",
                    err
                )
            });
        unreachable!("the _init_call call will never return since `tail_call` was set");
    }
}
