pub use crate::traits::psp34::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Flush,
    },
};
pub use derive::PSP34Storage;
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};

use brush::traits::Balance;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP34Data {
    pub token_owner: StorageHashMap<Id, AccountId>,
    pub token_approvals: StorageHashMap<Id, AccountId>,
    pub owned_tokens_count: StorageHashMap<AccountId, u32>,
    pub operator_approvals: StorageHashMap<(AccountId, AccountId), bool>,
    pub total_supply: Balance,
}

declare_storage_trait!(PSP34Storage, PSP34Data);

impl<T: PSP34Storage + Flush> PSP34 for T {
    default fn collection_id(&self) -> Id {
        // TODO: make it work
        // Id::Bytes(Self::env().account_id().as_ref().to_vec())
        todo!()
    }

    default fn balance_of(&self, owner: AccountId) -> u32 {
        self.get().owned_tokens_count.get(&owner).cloned().unwrap_or(0)
    }

    default fn owner_of(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    default fn get_approved(&self, id: Id) -> Option<AccountId> {
        self.get().token_approvals.get(&id).cloned()
    }

    default fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self._approved_for_all(owner, operator)
    }

    default fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP34Error> {
        let caller = Self::env().caller();
        self._approve_for_all(caller, operator, approved)?;
        Ok(())
    }

    default fn approve(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._approve_for(to, id)?;
        Ok(())
    }

    default fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        self._transfer_token_from(Self::env().caller(), to, id, data)?;
        Ok(())
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34Error> {
        self._transfer_token_from(from, to, id, data)?;
        Ok(())
    }

    default fn total_supply(&self) -> Balance {
        self.get().total_supply
    }
}

pub trait PSP34Internal {
    /// Emits transfer event. This method must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id);

    /// Emits approval event. This method must be implemented in derived implementation
    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id);

    /// Emits approval for all event. This method must be implemented in derived implementation
    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool);

    /// Event is emitted when an attribute is set for a token.
    fn _emit_attribute_set_event(&self, _id: Id, _key: Vec<u8>, _data: Vec<u8>);

    /// Approves or disapproves the operator to transfer all tokens of the caller.
    fn _approve_for_all(&mut self, owner: AccountId, operator: AccountId, approved: bool) -> Result<(), PSP34Error>;

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error>;

    /// Returns the owner of the token.
    fn _owner_of(&self, id: &Id) -> Option<AccountId>;

    fn _approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool;

    /// Gets an operator on other Account's behalf.
    fn _transfer_token_from(&mut self, from: AccountId, to: AccountId, id: Id, data: Vec<u8>)
        -> Result<(), PSP34Error>;

    /// Transfers token `id` `from` the sender to the `to` AccountId.
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id
    ) -> Result<(), PSP34Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id
    ) -> Result<(), PSP34Error>;

    /// Child contract can override that if they don't want to do a cross call
    fn _do_safe_transfer_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34Error>;

    fn _add_token(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _remove_token(&mut self, from: AccountId, id: &Id) -> Result<(), PSP34Error>;

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error>;
}

impl<T: PSP34Storage + Flush> PSP34Internal for T {
    default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id) {}

    default fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id) {}

    default fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    default fn _emit_attribute_set_event(&self, _id: Id, _key: Vec<u8>, _data: Vec<u8>) {}

    default fn _approve_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), PSP34Error> {
        if owner == operator {
            return Err(PSP34Error::SelfApprove)
        }
        self.get_mut().operator_approvals.insert((owner, operator), approved);
        self._emit_approval_for_all_event(owner, operator, approved);
        Ok(())
    }

    default fn _approve_for(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        let owner = self.get().token_owner.get(&id).cloned();

        if owner.is_none() {
            return Err(PSP34Error::TokenNotExists)
        }
        let owner = owner.unwrap();
        let caller = Self::env().caller();

        if owner == to {
            return Err(PSP34Error::SelfApprove)
        }

        if owner != caller && !self._approved_for_all(owner, caller) {
            return Err(PSP34Error::NotApproved)
        };

        self.get_mut().token_approvals.insert(id.clone(), to);
        self._emit_approval_event(caller, to, id);
        Ok(())
    }

    default fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.get().token_owner.get(id).cloned()
    }

    default fn _approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self.get()
            .operator_approvals
            .get(&(owner, operator))
            .unwrap_or(&false)
            .clone()
    }

    default fn _transfer_token_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34Error> {
        let owner = self.get().token_owner.get(&id).cloned();

        if owner.is_none() {
            return Err(PSP34Error::TokenNotExists)
        }

        self._before_token_transfer(Some(&from), Some(&to), &id)?;

        let owner = owner.unwrap();
        let caller = Self::env().caller();
        if owner != caller
            && Some(caller) != self.get().token_approvals.get(&id).cloned()
            && !self._approved_for_all(owner, caller)
        {
            return Err(PSP34Error::NotApproved)
        }

        self._remove_token(from, &id)?;
        self._do_safe_transfer_check(Self::env().caller(), from, to, id.clone(), data)?;
        self._add_token(to.clone(), id.clone())?;
        self._emit_transfer_event(Some(from), Some(to), id.clone());
        self._after_token_transfer(Some(&from), Some(&to), &id)?;
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id
    ) -> Result<(), PSP34Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id
    ) -> Result<(), PSP34Error> {
        Ok(())
    }

    default fn _do_safe_transfer_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34Error> {
        self.flush();
        let result = match PSP34ReceiverRef::before_received_builder(&to, operator, from, id, data).fire() {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => {
                match e {
                    // `NotCallable` means that the receiver is not a contract.

                    // `CalleeTrapped` means that the receiver has no method called `before_received` or it failed inside.
                    // First case is expected. Second - not. But we can't tell them apart so it is a positive case for now.
                    // https://github.com/paritytech/ink/issues/1002
                    EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
                    _ => {
                        Err(PSP34Error::SafeTransferCheckFailed(String::from(
                            "Error during call to receiver",
                        )))
                    }
                }
            }
        };
        self.load();
        result
    }

    default fn _add_token(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        let to_balance = self.get_mut().owned_tokens_count.get_mut(&to).cloned().unwrap_or(0);
        self.get_mut().owned_tokens_count.insert(to.clone(), to_balance + 1);
        self.get_mut().total_supply += 1;

        self.get_mut().token_owner.insert(id, to);
        Ok(())
    }

    default fn _remove_token(&mut self, from: AccountId, id: &Id) -> Result<(), PSP34Error> {
        self.get_mut().token_approvals.take(id);
        self.get_mut().token_owner.take(id);

        let from_balance = self.get_mut().owned_tokens_count.get_mut(&from).cloned().unwrap_or(1);
        self.get_mut().owned_tokens_count.insert(from, from_balance - 1);
        self.get_mut().total_supply -= 1;
        Ok(())
    }

    default fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        if self.get_mut().token_owner.get(&id).is_some() {
            return Err(PSP34Error::TokenExists)
        }

        self._before_token_transfer(None, Some(&to), &id)?;

        self._add_token(to, id.clone())?;
        self._emit_transfer_event(None, Some(to), id.clone());

        self._after_token_transfer(None, Some(&to), &id)?;
        Ok(())
    }

    default fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
        if self.get_mut().token_owner.get(&id).is_none() {
            return Err(PSP34Error::TokenNotExists)
        }

        self._before_token_transfer(Some(&from), None, &id)?;

        self._remove_token(from, &id)?;
        self._emit_transfer_event(Some(from), None, id.clone());

        self._after_token_transfer(Some(&from), None, &id)?;
        Ok(())
    }
}
