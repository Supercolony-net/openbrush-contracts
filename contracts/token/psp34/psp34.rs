pub use crate::traits::psp34::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Flush,
    },
};
pub use derive::PSP34Storage;
use ink_env::{
    CallFlags,
    Error as EnvError,
};
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::Mapping;

use brush::traits::Balance;

type Owner = AccountId;
type Operator = AccountId;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP34Data");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP34Data {
    pub token_owner: Mapping<Id, Owner>,
    pub owned_tokens_count: Mapping<Owner, u32>,
    pub operator_approvals: Mapping<(Owner, Operator, Option<Id>), ()>,
    pub total_supply: Balance,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP34Storage, PSP34Data);

impl<T: PSP34Storage + Flush> PSP34 for T {
    default fn collection_id(&self) -> Id {
        let account_id = Self::env().account_id();
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }

    default fn balance_of(&self, owner: AccountId) -> u32 {
        self._balance_of(&owner)
    }

    default fn owner_of(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    default fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
        self._allowance(&owner, &operator, &id)
    }

    default fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        self._approve_for(operator, id, approved)?;
        Ok(())
    }

    default fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        self._transfer_token(to, id, data)?;
        Ok(())
    }

    default fn total_supply(&self) -> Balance {
        self._total_supply()
    }
}

pub trait PSP34Internal {
    /// Emits transfer event. This method must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id);

    /// Emits approval event. This method must be implemented in derived implementation
    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, approved: bool);

    /// Event is emitted when an attribute is set for a token.
    fn _emit_attribute_set_event(&self, _id: Id, _key: Vec<u8>, _data: Vec<u8>);

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error>;

    /// Returns the owner of the token.
    fn _owner_of(&self, id: &Id) -> Option<AccountId>;

    /// Gets an operator on other Account's behalf.
    fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    /// Transfers token `id` `from` the sender to the `to` AccountId.
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error>;

    /// Child contract can override that if they don't want to do a cross call
    fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        id: &Id,
        data: &Vec<u8>,
    ) -> Result<(), PSP34Error>;

    fn _add_token(&mut self, to: &AccountId, id: &Id) -> Result<(), PSP34Error>;

    fn _remove_token(&mut self, from: &AccountId, id: &Id) -> Result<(), PSP34Error>;

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _balance_of(&self, owner: &AccountId) -> u32;

    fn _total_supply(&self) -> Balance;

    fn _allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<Id>) -> bool;

    fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error>;
}

impl<T: PSP34Storage + Flush> PSP34Internal for T {
    default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id) {}

    default fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, _approved: bool) {}

    default fn _emit_attribute_set_event(&self, _id: Id, _key: Vec<u8>, _data: Vec<u8>) {}

    default fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        let mut caller = Self::env().caller();

        if id.is_some() {
            let maybe_owner = self.get().token_owner.get(id.as_ref().unwrap());

            if maybe_owner.is_none() {
                return Err(PSP34Error::TokenNotExists)
            }
            let owner = maybe_owner.unwrap();

            if approved && owner == to {
                return Err(PSP34Error::SelfApprove)
            }

            if owner != caller && !self._allowance(&owner, &caller, &None) {
                return Err(PSP34Error::NotApproved)
            };
            caller = owner;
        }

        if approved {
            self.get_mut().operator_approvals.insert((&caller, &to, &id), &());
        } else {
            self.get_mut().operator_approvals.remove((&caller, &to, &id));
        }
        self._emit_approval_event(caller, to, id, approved);

        Ok(())
    }

    default fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.get().token_owner.get(&id)
    }

    default fn _allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<Id>) -> bool {
        self.get().operator_approvals.get((owner, operator, &None)).is_some()
            || id != &None && self.get().operator_approvals.get((owner, operator, id)).is_some()
    }

    default fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        self.get().token_owner.get(&id).ok_or(PSP34Error::TokenNotExists)
    }

    default fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        let owner = self._check_token_exists(&id)?;
        let caller = Self::env().caller();
        let id = Some(id);

        if owner != caller && !self._allowance(&owner, &caller, &id) {
            return Err(PSP34Error::NotApproved)
        }

        self.get_mut().operator_approvals.remove((&owner, &caller, &id));

        let id = id.unwrap();

        self._before_token_transfer(Some(&owner), Some(&to), &id)?;
        self._remove_token(&owner, &id)?;
        self._do_safe_transfer_check(&caller, &owner, &to, &id, &data)?;
        self._add_token(&to, &id)?;
        self._after_token_transfer(Some(&owner), Some(&to), &id)?;
        self._emit_transfer_event(Some(owner), Some(to), id);
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _id: &Id,
    ) -> Result<(), PSP34Error> {
        Ok(())
    }

    default fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        id: &Id,
        data: &Vec<u8>,
    ) -> Result<(), PSP34Error> {
        self.flush();
        let builder =
            PSP34ReceiverRef::before_received_builder(to, operator.clone(), from.clone(), id.clone(), data.clone())
                .call_flags(CallFlags::default().set_allow_reentry(true));
        let result = match builder.fire() {
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

    default fn _add_token(&mut self, to: &AccountId, id: &Id) -> Result<(), PSP34Error> {
        let to_balance = self.get_mut().owned_tokens_count.get(to).unwrap_or(0);
        self.get_mut().owned_tokens_count.insert(to, &(to_balance + 1));
        self.get_mut().total_supply += 1;

        self.get_mut().token_owner.insert(id, to);
        Ok(())
    }

    default fn _remove_token(&mut self, from: &AccountId, id: &Id) -> Result<(), PSP34Error> {
        self.get_mut().token_owner.remove(id);

        let from_balance = self.get_mut().owned_tokens_count.get(from).unwrap_or(0);
        self.get_mut().owned_tokens_count.insert(from, &(from_balance - 1));
        self.get_mut().total_supply -= 1;
        Ok(())
    }

    default fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        if self.get_mut().token_owner.get(&id).is_some() {
            return Err(PSP34Error::TokenExists)
        }
        self._before_token_transfer(None, Some(&to), &id)?;

        self._add_token(&to, &id)?;
        self._after_token_transfer(None, Some(&to), &id)?;
        self._emit_transfer_event(None, Some(to), id);

        Ok(())
    }

    default fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._check_token_exists(&id)?;

        self._before_token_transfer(Some(&from), None, &id)?;

        self._remove_token(&from, &id)?;
        self._after_token_transfer(Some(&from), None, &id)?;
        self._emit_transfer_event(Some(from), None, id);
        Ok(())
    }

    fn _balance_of(&self, owner: &AccountId) -> u32 {
        self.get().owned_tokens_count.get(owner).unwrap_or(0)
    }

    fn _total_supply(&self) -> Balance {
        self.get().total_supply
    }
}
