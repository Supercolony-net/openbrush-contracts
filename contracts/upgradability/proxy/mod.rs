pub use crate::{
    ownable::*,
    traits::proxy::*
};
use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::Hash,
};

pub use derive::ProxyStorage;
use ink_env::call::DelegateCall;
use ink_storage::traits::{SpreadAllocate, SpreadLayout};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct ProxyData {
    pub forward_to: Hash,
}

declare_storage_trait!(ProxyStorage, ProxyData);

#[modifier_definition]
pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: OwnableStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<OwnableError>,
{
    if instance.get().owner != T::env().caller() {
        return Err(From::from(OwnableError::CallerIsNotOwner));
    }
    body(instance)
}

impl<T: OwnableStorage + ProxyStorage> Proxy for T {
    default fn get_delegate_code(&self) -> Hash {
        ProxyStorage::get(self).forward_to
    }

    #[modifiers(only_owner)]
    default fn change_delegate_code(&mut self, new_forward_to: Hash) -> Result<(), ProxyError> {
        ProxyStorage::get_mut(self).forward_to = new_forward_to;
        Ok(())
    }
}

pub trait ProxyInternal {
    fn _init_with_forward_to(&mut self, forward_to: Hash);

    fn _fallback(&self);
}


impl<T: OwnableStorage + ProxyStorage> ProxyInternal for T {
    default fn _init_with_forward_to(&mut self, forward_to: Hash) {
        ProxyStorage::get_mut(self).forward_to = forward_to;
    }

    default fn _fallback(&self) {
        ink_env::call::build_call::<ink_env::DefaultEnvironment>()
        .call_type(DelegateCall::new().code_hash(self.get_delegate_code()))
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
                self.get_delegate_code(), err
            )
        });
    unreachable!(
        "the forwarded call will never return since `tail_call` was set"
    );
    }
}


