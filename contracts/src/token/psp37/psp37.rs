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
    psp37,
    psp37::balances,
    traits::psp37::*,
};
pub use psp37::{
    Internal as _,
    Transfer as _,
};

use core::result::Result;
use ink::{
    env::CallFlags,
    prelude::{
        vec,
        vec::Vec,
    },
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
        AccountIdExt,
        Balance,
        OccupiedStorage,
        Storage,
        String,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data<B = balances::Balances>
where
    B: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>,
{
    pub balances: B,
    pub operator_approvals: Mapping<
        (AccountId, AccountId, Option<Id>),
        Balance,
        ApprovalsKey, // optimization
    >,
    pub _reserved: Option<()>,
}

pub struct ApprovalsKey;

impl<'a> TypeGuard<'a> for ApprovalsKey {
    type Type = &'a (&'a AccountId, &'a AccountId, &'a Option<&'a Id>);
}

impl<B, T> PSP37 for T
where
    B: balances::BalancesManager,
    B: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>,
    T: Storage<Data<B>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<B>>,
{
    default fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance {
        self.data().balances.balance_of(&owner, &id.as_ref())
    }

    default fn total_supply(&self, id: Option<Id>) -> Balance {
        self.data().balances.total_supply(&id.as_ref())
    }

    default fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
        match id {
            None => self._get_allowance(&owner, &operator, &None),
            Some(id) => self._get_allowance(&owner, &operator, &Some(&id)),
        }
    }

    default fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
        self._approve_for(operator, id, value)
    }

    default fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP37Error> {
        self._transfer_from(Self::env().caller(), to, id, value, data)
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self._transfer_from(from, to, id, value, data)
    }
}

pub trait Internal {
    /// Those methods must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id, _amount: Balance);
    fn _emit_transfer_batch_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_amounts: Vec<(Id, Balance)>,
    );
    fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, value: Balance);

    /// Creates `amount` tokens of token type `id` to `to`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `TransferToZeroAddress` error if `to` is zero account.
    fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;

    /// Destroys `amount` tokens of token type `id` from `from`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if transfer is not approved.
    ///
    /// Returns with `InsufficientBalance` error if `from` doesn't contain enough balance.
    fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;

    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error>;

    fn _get_allowance(&self, account: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance;

    fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error>;

    fn _decrease_allowance(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Id,
        value: Balance,
    ) -> Result<(), PSP37Error>;

    fn _transfer_token(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: Id,
        amount: Balance,
        data: &Vec<u8>,
    ) -> Result<(), PSP37Error>;

    fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        ids_amounts: &Vec<(Id, Balance)>,
        data: &Vec<u8>,
    ) -> Result<(), PSP37Error>;
}

impl<B, T> Internal for T
where
    B: balances::BalancesManager,
    B: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>,
    T: Storage<Data<B>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<B>>,
{
    default fn _emit_transfer_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _id: Id,
        _amount: Balance,
    ) {
    }
    default fn _emit_transfer_batch_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_amounts: Vec<(Id, Balance)>,
    ) {
    }
    default fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, _value: Balance) {}

    default fn _mint_to(&mut self, to: AccountId, mut ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
        if to.is_zero() {
            return Err(PSP37Error::TransferToZeroAddress)
        }
        if ids_amounts.is_empty() {
            return Ok(())
        }

        self._before_token_transfer(None, Some(&to), &ids_amounts)?;

        for (id, amount) in &ids_amounts {
            self.data().balances.increase_balance(&to, id, amount, true)?;
        }

        self._after_token_transfer(None, Some(&to), &ids_amounts)?;

        if ids_amounts.len() == 1 {
            let (id, amount) = unsafe { ids_amounts.pop().unwrap_unchecked() };
            self._emit_transfer_event(None, Some(to), id, amount);
        } else {
            self._emit_transfer_batch_event(None, Some(to), ids_amounts);
        }

        Ok(())
    }

    default fn _burn_from(&mut self, from: AccountId, mut ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
        self._before_token_transfer(Some(&from), None, &ids_amounts)?;

        if ids_amounts.is_empty() {
            return Ok(())
        }

        for (id, amount) in ids_amounts.iter() {
            self.data().balances.decrease_balance(&from, id, amount, true)?;
        }

        self._after_token_transfer(Some(&from), None, &ids_amounts)?;

        if ids_amounts.len() == 1 {
            let (id, amount) = unsafe { ids_amounts.pop().unwrap_unchecked() };
            self._emit_transfer_event(Some(from), None, id, amount);
        } else {
            self._emit_transfer_batch_event(Some(from), None, ids_amounts);
        }

        Ok(())
    }

    default fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error> {
        let operator = Self::env().caller();
        let ids_amounts = vec![(id.clone(), value)];

        if to.is_zero() {
            return Err(PSP37Error::TransferToZeroAddress)
        }

        if from != operator && self._get_allowance(&from, &operator, &Some(&id)) < value {
            return Err(PSP37Error::NotAllowed)
        }

        self._before_token_transfer(Some(&from), Some(&to), &ids_amounts)?;
        self._decrease_allowance(&from, &operator, &id, value)?;
        self._transfer_token(&from, &to, id.clone(), value, &data)?;
        self._after_token_transfer(Some(&from), Some(&to), &ids_amounts)?;
        self._emit_transfer_event(Some(from), Some(to), id, value);
        Ok(())
    }

    default fn _get_allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance {
        return match self.data().operator_approvals.get(&(owner, operator, &None)) {
            None => self.data().operator_approvals.get(&(owner, operator, id)).unwrap_or(0),
            _ => Balance::MAX,
        }
    }

    fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
        let caller = Self::env().caller();

        if caller == operator {
            return Err(PSP37Error::SelfApprove)
        }

        if let Some(id) = &id {
            if value == 0 {
                self.data().operator_approvals.remove(&(&caller, &operator, &Some(id)));
            } else {
                self.data()
                    .operator_approvals
                    .insert(&(&caller, &operator, &Some(id)), &value);
            }
        } else {
            if value == 0 {
                self.data().operator_approvals.remove(&(&caller, &operator, &None));
            } else {
                self.data()
                    .operator_approvals
                    .insert(&(&caller, &operator, &None), &Balance::MAX);
            }
        }

        self._emit_approval_event(caller, operator, id, value);

        Ok(())
    }

    fn _decrease_allowance(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Id,
        value: Balance,
    ) -> Result<(), PSP37Error> {
        if owner == operator {
            return Ok(())
        }

        let initial_allowance = self._get_allowance(owner, operator, &Some(id));

        if initial_allowance == Balance::MAX {
            return Ok(())
        }

        if initial_allowance < value {
            return Err(PSP37Error::InsufficientBalance)
        }

        self.data()
            .operator_approvals
            .insert(&(owner, operator, &Some(id)), &(initial_allowance - value));

        Ok(())
    }

    fn _transfer_token(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: Id,
        value: Balance,
        data: &Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self.data().balances.decrease_balance(from, &id, &value, false)?;
        self._do_safe_transfer_check(&Self::env().caller(), from, to, &vec![(id.clone(), value)], &data)?;
        self.data().balances.increase_balance(to, &id, &value, false)?;
        Ok(())
    }

    default fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        ids_amounts: &Vec<(Id, Balance)>,
        data: &Vec<u8>,
    ) -> Result<(), PSP37Error> {
        self.flush();
        let builder = PSP37ReceiverRef::before_received_builder(
            to,
            operator.clone(),
            from.clone(),
            ids_amounts.clone(),
            data.clone(),
        )
        .call_flags(CallFlags::default().set_allow_reentry(true));
        let result = match builder.fire() {
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(e))) => Err(e.into()),
            // `NotCallable` means that the receiver is not a contract.
            Err(ink::env::Error::NotCallable) => Ok(()),
            // Means unknown method
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            _ => {
                Err(PSP37Error::SafeTransferCheckFailed(String::from(
                    "Error during call to receiver",
                )))
            }
        };
        self.load();
        result
    }
}

pub trait Transfer {
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error>;
}

impl<B, T> Transfer for T
where
    B: balances::BalancesManager,
    B: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<453953544, ManualKey<{ STORAGE_KEY }>>, Type = B>,
    T: Storage<Data<B>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<B>>,
{
    default fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP37Error> {
        Ok(())
    }
}
