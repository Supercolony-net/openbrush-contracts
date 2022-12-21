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
    psp34,
    psp34::balances,
    traits::psp34::*,
};
pub use psp34::{
    Internal as _,
    Transfer as _,
};

use crate::psp34::{
    Operator,
    Owner,
};
use ink::{
    env::CallFlags,
    prelude::vec::Vec,
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
};
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        OccupiedStorage,
        Storage,
        String,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data<B = balances::Balances>
where
    B: Storable
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>
        + StorableHint<ManualKey<{ STORAGE_KEY }>>,
{
    pub token_owner: Mapping<Id, Owner>,
    pub operator_approvals: Mapping<(Owner, Operator, Option<Id>), (), ApprovalsKey>,
    pub balances: B,
    pub _reserved: Option<()>,
}

pub struct ApprovalsKey;

impl<'a> TypeGuard<'a> for ApprovalsKey {
    type Type = &'a (&'a Owner, &'a Operator, &'a Option<&'a Id>);
}

impl<B, T> PSP34 for T
where
    B: balances::BalancesManager,
    B: Storable
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>
        + StorableHint<ManualKey<{ STORAGE_KEY }>>,
    T: Storage<Data<B>>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data<B>>,
{
    default fn collection_id(&self) -> Id {
        let account_id = Self::env().account_id();
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }

    default fn balance_of(&self, owner: AccountId) -> u32 {
        self.data().balances.balance_of(&owner)
    }

    default fn owner_of(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    default fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
        self._allowance(&owner, &operator, &id.as_ref())
    }

    default fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        let _a = &self.data().balances;
        self._approve_for(operator, id, approved)
    }

    default fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        self._transfer_token(to, id, data)
    }

    default fn total_supply(&self) -> Balance {
        self.data().balances.total_supply()
    }
}

pub trait Internal {
    /// Those methods must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id);
    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, _approved: bool);

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

impl<B, T> Internal for T
where
    B: balances::BalancesManager,
    B: Storable
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>
        + StorableHint<ManualKey<{ STORAGE_KEY }>>,
    T: Storage<Data<B>>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data<B>>,
{
    default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id) {}
    default fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Option<Id>, _approved: bool) {}

    default fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
        let mut caller = Self::env().caller();

        if let Some(id) = &id {
            let owner = self.data().token_owner.get(id).ok_or(PSP34Error::TokenNotExists)?;

            if approved && owner == to {
                return Err(PSP34Error::SelfApprove)
            }

            if owner != caller && !self._allowance(&owner, &caller, &None) {
                return Err(PSP34Error::NotApproved)
            };
            caller = owner;
        }

        if approved {
            self.data()
                .operator_approvals
                .insert(&(&caller, &to, &id.as_ref()), &());
        } else {
            self.data().operator_approvals.remove(&(&caller, &to, &id.as_ref()));
        }
        self._emit_approval_event(caller, to, id, approved);

        Ok(())
    }

    default fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.data().token_owner.get(id)
    }

    default fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
        let owner = self._check_token_exists(&id)?;
        let caller = Self::env().caller();

        if owner != caller && !self._allowance(&owner, &caller, &Some(&id)) {
            return Err(PSP34Error::NotApproved)
        }

        self._before_token_transfer(Some(&owner), Some(&to), &id)?;

        self.data().operator_approvals.remove(&(&owner, &caller, &Some(&id)));
        self.data().balances.decrease_balance(&owner, &id, false);
        self.data().token_owner.remove(&id);

        self._do_safe_transfer_check(&caller, &owner, &to, &id, &data)?;

        self.data().balances.increase_balance(&to, &id, false);
        self.data().token_owner.insert(&id, &to);
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
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(e))) => Err(e.into()),
            // Means unknown method
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            _ => {
                Err(PSP34Error::SafeTransferCheckFailed(String::from(
                    "Error during call to receiver",
                )))
            }
        };
        self.load();
        result
    }

    default fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        if self.data().token_owner.get(&id).is_some() {
            return Err(PSP34Error::TokenExists)
        }
        self._before_token_transfer(None, Some(&to), &id)?;

        self.data().balances.increase_balance(&to, &id, true);
        self.data().token_owner.insert(&id, &to);
        self._after_token_transfer(None, Some(&to), &id)?;
        self._emit_transfer_event(None, Some(to), id);

        Ok(())
    }

    default fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
        self._check_token_exists(&id)?;

        self._before_token_transfer(Some(&from), None, &id)?;

        self.data().token_owner.remove(&id);
        self.data().balances.decrease_balance(&from, &id, true);
        self._after_token_transfer(Some(&from), None, &id)?;
        self._emit_transfer_event(Some(from), None, id);
        Ok(())
    }

    default fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
        self.data().operator_approvals.get(&(owner, operator, &None)).is_some()
            || id != &None && self.data().operator_approvals.get(&(owner, operator, id)).is_some()
    }

    default fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        self.data().token_owner.get(&id).ok_or(PSP34Error::TokenNotExists)
    }
}

pub trait Transfer {
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

impl<B, T> Transfer for T
where
    B: balances::BalancesManager,
    B: Storable
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>
        + StorableHint<ManualKey<{ STORAGE_KEY }>>,
    T: Storage<Data<B>>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data<B>>,
{
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
