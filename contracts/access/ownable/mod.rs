pub use crate::traits::ownable::*;
use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        ZERO_ADDRESS,
    },
};
pub use derive::OwnableStorage;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::OwnableData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct OwnableData {
    pub owner: AccountId,
}

declare_storage_trait!(OwnableStorage, OwnableData);

/// Throws if called by any account other than the owner.
#[modifier_definition]
pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: OwnableStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<OwnableError>,
{
    if instance.get().owner != T::env().caller() {
        return Err(From::from(OwnableError::CallerIsNotOwner))
    }
    body(instance)
}

impl<T: OwnableStorage> Ownable for T {
    default fn owner(&self) -> AccountId {
        self.get().owner.clone()
    }

    #[modifiers(only_owner)]
    default fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = ZERO_ADDRESS.into();
        self._emit_ownership_transferred_event(Some(old_owner), None);
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        if new_owner.is_zero() {
            return Err(OwnableError::NewOwnerIsZero)
        }
        let old_owner = self.get().owner.clone();
        self.get_mut().owner = new_owner.clone();
        self._emit_ownership_transferred_event(Some(old_owner), Some(new_owner));
        Ok(())
    }
}

pub trait OwnableInternal {
    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous_owner: Option<AccountId>, _new_owner: Option<AccountId>);

    fn _init_with_owner(&mut self, owner: AccountId);
}

impl<T: OwnableStorage> OwnableInternal for T {
    /// User must override this method in their contract.
    default fn _emit_ownership_transferred_event(
        &self,
        _previous_owner: Option<AccountId>,
        _new_owner: Option<AccountId>,
    ) {
    }

    default fn _init_with_owner(&mut self, owner: AccountId) {
        self.get_mut().owner = owner;
        self._emit_ownership_transferred_event(None, Some(owner));
    }
}
