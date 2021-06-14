use brush::{
    traits::{InkStorage, AccountId},
    define_getters,
};
use crate::traits::OwnableError;

pub trait OwnableStorage: InkStorage {
    define_getters!(_owner, _owner_mut, AccountId);
}

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Ownable: OwnableStorage {
    fn only_owner(&self) {
        assert_eq!(self._owner(), &Self::env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
    }

    fn _init_with_owner(&mut self, owner: AccountId) {
        *self._owner_mut() = owner;
        // TODO: Emit event
    }

    fn owner(&self) -> AccountId {
        self._owner().clone()
    }

    fn renounce_ownership(&mut self) {
        self.only_owner();

        // TODO: Emit event
        *self._owner_mut() = ZERO_ADDRESS.into();
    }

    fn transfer_ownership(&mut self, new_owner: AccountId) {
        self.only_owner();
        assert_ne!(new_owner, ZERO_ADDRESS.into(), "{}", OwnableError::NewOwnerIsZero.as_ref());
        // TODO: Emit event
        *self._owner_mut() = new_owner;
    }
}
