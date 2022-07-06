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

pub use super::balances::*;
use crate::psp34::{
    Operator,
    Owner,
};
pub use crate::traits::psp34::*;
pub use derive::PSP34Storage;
use ink_env::{
    CallFlags,
    Error as EnvError,
};
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::traits::{
    SpreadAllocate,
    SpreadLayout,
};
use openbrush::{
    declare_storage_trait,
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Flush,
    },
};

pub const DATA_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP34Data");

#[derive(Default, Debug)]
#[openbrush::storage(DATA_KEY)]
pub struct PSP34Data<B = Balances>
where
    B: BalancesManager + SpreadLayout + SpreadAllocate,
{
    pub token_owner: Mapping<Id, Owner>,
    pub operator_approvals: Mapping<(Owner, Operator, Option<Id>), (), ApprovalsKey /* optimization */>,
    pub balances: B,
    pub _reserved: Option<()>,
}

pub struct ApprovalsKey;

impl<'a> TypeGuard<'a> for ApprovalsKey {
    type Type = &'a (&'a Owner, &'a Operator, &'a Option<&'a Id>);
}

declare_storage_trait!(PSP34Storage);

impl<B, T> PSP34 for T
where
    B: BalancesManager + SpreadLayout + SpreadAllocate,
    T: PSP34Storage<Data = PSP34Data<B>> + Flush,
{
    default fn collection_id(&self) -> Id {
        let account_id = Self::env().account_id();
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }

    default fn balance_of(&self, owner: AccountId) -> u32 {
        self.get().balances.balance_of(&owner)
    }

    default fn owner_of(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    default fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
        self._allowance(&owner, &operator, &id.as_ref())
    }

    default fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        self._approve_for(operator, id, approved)
    }

    default fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        self._transfer_token(to, id, data)
    }

    default fn total_supply(&self) -> Balance {
        self.get().balances.total_supply()
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

    /// Child contract can override that if they don't want to do a cross call
    fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        id: &Id,
        data: &Vec<u8>,
    ) -> Result<(), PSP34Error>;

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error>;

    fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool;

    fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error>;
}

impl<B, T> PSP34Internal for T
where
    B: BalancesManager + SpreadLayout + SpreadAllocate,
    T: PSP34Storage<Data = PSP34Data<B>> + Flush,
{
    default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id) {}

    default fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, _approved: bool) {}

    default fn _emit_attribute_set_event(&self, _id: Id, _key: Vec<u8>, _data: Vec<u8>) {}

    default fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        let mut caller = Self::env().caller();

        if let Some(id) = &id {
            let owner = self.get().token_owner.get(id).ok_or(PSP34Error::TokenNotExists)?;

            if approved && owner == to {
                return Err(PSP34Error::SelfApprove)
            }

            if owner != caller && !self._allowance(&owner, &caller, &None) {
                return Err(PSP34Error::NotApproved)
            };
            caller = owner;
        }

        if approved {
            self.get_mut()
                .operator_approvals
                .insert(&(&caller, &to, &id.as_ref()), &());
        } else {
            self.get_mut().operator_approvals.remove(&(&caller, &to, &id.as_ref()));
        }
        self._emit_approval_event(caller, to, id, approved);

        Ok(())
    }

    default fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.get().token_owner.get(&id)
    }

    default fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        let owner = self._check_token_exists(&id)?;
        let caller = Self::env().caller();

        if owner != caller && !self._allowance(&owner, &caller, &Some(&id)) {
            return Err(PSP34Error::NotApproved)
        }

        self._before_token_transfer(Some(&owner), Some(&to), &id)?;

        self.get_mut().operator_approvals.remove(&(&owner, &caller, &Some(&id)));
        self.get_mut().balances.decrease_balance(&owner, &id, false);
        self.get_mut().token_owner.remove(&id);

        self._do_safe_transfer_check(&caller, &owner, &to, &id, &data)?;

        self.get_mut().balances.increase_balance(&to, &id, false);
        self.get_mut().token_owner.insert(&id, &to);
        self._after_token_transfer(Some(&owner), Some(&to), &id)?;
        self._emit_transfer_event(Some(owner), Some(to), id);

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
        let b = builder.fire();
        let result = match b {
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

    default fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        if self.get_mut().token_owner.get(&id).is_some() {
            return Err(PSP34Error::TokenExists)
        }
        self._before_token_transfer(None, Some(&to), &id)?;

        self.get_mut().balances.increase_balance(&to, &id, true);
        self.get_mut().token_owner.insert(&id, &to);
        self._after_token_transfer(None, Some(&to), &id)?;
        self._emit_transfer_event(None, Some(to), id);

        Ok(())
    }

    default fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._check_token_exists(&id)?;

        self._before_token_transfer(Some(&from), None, &id)?;

        self.get_mut().token_owner.remove(&id);
        self.get_mut().balances.decrease_balance(&from, &id, true);
        self._after_token_transfer(Some(&from), None, &id)?;
        self._emit_transfer_event(Some(from), None, id);
        Ok(())
    }

    default fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
        self.get().operator_approvals.get(&(owner, operator, &None)).is_some()
            || id != &None && self.get().operator_approvals.get(&(owner, operator, id)).is_some()
    }

    default fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        self.get().token_owner.get(&id).ok_or(PSP34Error::TokenNotExists)
    }
}

pub trait PSP34Transfer {
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
}

impl<T> PSP34Transfer for T {
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
}
