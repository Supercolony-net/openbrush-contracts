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

pub use derive::DiamondStorage;
use ink_storage::Mapping;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::DiamondData");

// TODO: Add support of Erc165
#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct DiamondData {
    pub ownable: OwnableData,
    pub selector_to_hash: Mapping<Selector, Hash>,
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
}

pub trait DiamondInternal {
    fn _diamond_cut(&mut self, diamond_cut: Vec<FacetCut>, init: Option<InitCall>) -> Result<(), DiamondError>;

    fn _fallback(&self) -> !;

    fn _init_call(&self, call: InitCall) -> !;

    fn _add_function(&mut self, code_hash: Hash, selector: [u8; 4]) -> Result<(), DiamondError>;

    fn _replace_function(&mut self, code_hash: Hash, selector: [u8; 4]) -> Result<(), DiamondError>;

    fn _remove_function(&mut self, selector: [u8; 4]) -> Result<(), DiamondError>;
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
        let selector = ink_env::decode_input::<[u8; 4]>().unwrap_or_else(|_| panic!("Calldata error"));

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

    fn _add_function(&mut self, code_hash: Hash, selector: [u8; 4]) -> Result<(), DiamondError> {
        if self.get().selector_to_hash.get(selector).is_some() {
            return Err(DiamondError::FunctionAlreadyExists)
        }

        self.get_mut().selector_to_hash.insert(&selector, &code_hash);

        Ok(())
    }

    fn _replace_function(&mut self, code_hash: Hash, selector: [u8; 4]) -> Result<(), DiamondError> {
        if self
            .get()
            .selector_to_hash
            .get(selector)
            .ok_or(DiamondError::FunctionDoesNotExist)?
            != code_hash
        {
            return Err(DiamondError::ReplaceExisting)
        }

        self._remove_function(selector)?;
        self._add_function(code_hash, selector)
    }

    fn _remove_function(&mut self, selector: [u8; 4]) -> Result<(), DiamondError> {
        if self
            .get()
            .selector_to_hash
            .get(selector)
            .ok_or(DiamondError::FunctionDoesNotExist)?
            == self.get().self_hash
        {
            return Err(DiamondError::ImmutableFunction)
        }

        self.get().selector_to_hash.remove(selector);

        Ok(())
    }
}
