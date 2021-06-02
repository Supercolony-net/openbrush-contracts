use ::ink_env::{
    Environment,
    DefaultEnvironment,
};
use utils::{
    traits::{InkStorage, AccountId},
    define_getters,
};
use crate::traits::OwnableError;

pub trait OwnableStorage: InkStorage {
    define_getters!(_owner, _owner_mut, AccountId);
}

pub trait OwnableModifiers: OwnableStorage {
    fn only_owner(&self) -> Result<(), OwnableError> {
        if self._owner() != &Self::env().caller() {
            return Err(OwnableError::CallerIsNotOwner)
        }
        Ok(())
    }
}

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Ownable: OwnableModifiers {
    fn _init_with_owner(&mut self, owner: AccountId) {
        *self._owner_mut() = owner;
        // TODO: Emit event
    }

    fn _renounce_ownership(&mut self) -> Result<(), OwnableError> {
        self.only_owner()?;

        // TODO: Emit event
        *self._owner_mut() = ZERO_ADDRESS.into();
        Ok(())
    }

    fn _transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        self.only_owner()?;
        if new_owner == ZERO_ADDRESS.into() {
            return Err(OwnableError::NewOwnerIsZero)
        }
        // TODO: Emit event
        *self._owner_mut() = new_owner;
        Ok(())
    }
}
