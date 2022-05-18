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

pub use crate::traits::psp35::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Flush,
    },
};
use core::result::Result;
pub use derive::PSP35Storage;
use ink_env::{
    CallFlags,
    Error as EnvError,
};
use ink_prelude::{
    string::String,
    vec,
    vec::Vec,
};
use ink_storage::Mapping;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP35Data");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP35Data {
    pub balances: Mapping<(Id, AccountId), Balance>,
    pub operator_approvals: Mapping<(AccountId, AccountId, Option<Id>), Balance>,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP35Storage, PSP35Data);

impl<T: PSP35Storage + Flush> PSP35 for T {
    default fn balance_of(&self, owner: AccountId, id: Id) -> Balance {
        self._balance_of_or_zero(owner, id)
    }

    default fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
        self._get_allowance(owner, operator, id)
    }

    default fn approve(&mut self, operator: AccountId, token: Option<(Id, Balance)>) -> Result<(), PSP35Error> {
        self._approve_for(operator, token)?;
        Ok(())
    }

    default fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP35Error> {
        let caller = Self::env().caller();

        self.before_received(caller, caller, vec![(id, value)], data.clone())?;
        self._transfer_token(caller, to, id, value, data)?;

        self._emit_transfer_event(Some(caller), Some(to), id, value);
        Ok(())
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error> {
        let operator = Self::env().caller();

        self._transfer_guard(operator, from, to, id, value)?;
        self.before_received(operator, from, vec![(id, value)], data.clone())?;
        self._transfer_token(from, to, id, value, data)?;

        self._emit_transfer_event(Some(from), Some(to), id, value);
        Ok(())
    }
}

pub trait PSP35Internal {
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
    fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP35Error>;

    /// Destroys `amount` tokens of token type `id` from `from`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if transfer is not approved.
    ///
    /// Returns with `InsufficientBalance` error if `from` doesn't contain enough balance.
    fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP35Error>;

    fn _transfer_guard(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) -> Result<(), PSP35Error>;

    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance;

    fn _increase_receiver_balance(&mut self, to: AccountId, id: Id, amount: Balance);

    fn _decrease_sender_balance(&mut self, from: AccountId, id: Id, amount: Balance) -> Result<(), PSP35Error>;

    fn _get_allowance(&self, account: AccountId, operator: AccountId, id: Option<Id>) -> Balance;

    fn _approve_for(&mut self, operator: AccountId, token: Option<(Id, Balance)>) -> Result<(), PSP35Error>;

    fn _increase_allowance(&mut self, operator: AccountId, id: Id, value: Balance) -> Result<(), PSP35Error>;

    fn _decrease_allowance(&mut self, operator: AccountId, id: Id, value: Balance) -> Result<(), PSP35Error>;

    fn _transfer_token(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error>;

    fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        ids_amounts: &Vec<(Id, Balance)>,
        data: &Vec<u8>,
    ) -> Result<(), PSP35Error>;
}

impl<T: PSP35Storage + Flush> PSP35Internal for T {
    default fn _emit_transfer_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _id: Id,
        _amount: Balance,
    ) {
    }

    fn _emit_transfer_batch_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_amounts: Vec<(Id, Balance)>,
    ) {
    }

    default fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, _value: Balance) {}

    default fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP35Error> {
        let operator = Self::env().caller();

        if to.is_zero() {
            return Err(PSP35Error::TransferToZeroAddress)
        }
        if ids_amounts.is_empty() {
            return Ok(())
        }

        self.before_received(operator, [0; 32].into(), ids_amounts.clone(), Vec::new())?;

        for (id, amount) in ids_amounts.iter() {
            self._increase_receiver_balance(to, id.clone(), amount.clone());
        }

        if ids_amounts.len() == 1 {
            self._emit_transfer_event(None, Some(to), ids_amounts[0].0, ids_amounts[0].1);
        } else {
            self._emit_transfer_batch_event(None, Some(to), ids_amounts);
        }

        Ok(())
    }

    default fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP35Error> {
        let operator = Self::env().caller();

        self.before_received(operator, from, ids_amounts.clone(), Vec::new())?;

        if ids_amounts.is_empty() {
            return Ok(())
        }

        for (id, amount) in ids_amounts.iter() {
            self._decrease_sender_balance(from, id.clone(), amount.clone())?;
        }

        if ids_amounts.len() == 1 {
            self._emit_transfer_event(Some(from), None, ids_amounts[0].0, ids_amounts[0].1);
        } else {
            self._emit_transfer_batch_event(Some(from), None, ids_amounts);
        }

        Ok(())
    }

    default fn _transfer_guard(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
    ) -> Result<(), PSP35Error> {
        if to.is_zero() {
            return Err(PSP35Error::TransferToZeroAddress)
        }

        let id_opt = Some(id);

        if from != operator && self._get_allowance(from, operator, id_opt) < value {
            return Err(PSP35Error::NotAllowed)
        }
        Ok(())
    }

    default fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self.get().balances.get((&id, &owner)).unwrap_or(0)
    }

    default fn _increase_receiver_balance(&mut self, to: AccountId, id: Id, amount: Balance) {
        let to_balance = self.get_mut().balances.get((&id, &to)).unwrap_or(0);
        self.get_mut().balances.insert((&id, &to), &(to_balance + amount));
    }

    default fn _decrease_sender_balance(&mut self, from: AccountId, id: Id, amount: Balance) -> Result<(), PSP35Error> {
        let balance = self.balance_of(from, id);
        if balance < amount {
            return Err(PSP35Error::InsufficientBalance)
        }

        self.get_mut().balances.insert((&id, &from), &(balance - amount));
        Ok(())
    }

    default fn _get_allowance(&self, account: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
        let approval_for_all = self
            .get()
            .operator_approvals
            .get(&(account, operator, None))
            .unwrap_or(0);
        let approval_for_token = self.get().operator_approvals.get(&(account, operator, id)).unwrap_or(0);

        return if approval_for_all != 0 {
            approval_for_all
        } else {
            approval_for_token
        }
    }

    fn _approve_for(&mut self, operator: AccountId, token: Option<(Id, Balance)>) -> Result<(), PSP35Error> {
        let caller = Self::env().caller();

        if caller == operator {
            return Err(PSP35Error::NotAllowed)
        }

        let (id, value) = match token {
            Some((token_id, amount)) => ((Some(token_id)), amount),
            None => (None, Balance::MAX),
        };

        self.get_mut()
            .operator_approvals
            .insert(&(caller, operator, id), &value);

        self._emit_approval_event(caller, operator, id, value);

        Ok(())
    }

    fn _increase_allowance(&mut self, operator: AccountId, id: Id, value: Balance) -> Result<(), PSP35Error> {
        let caller = Self::env().caller();

        if caller == operator {
            return Err(PSP35Error::NotAllowed)
        }

        let new_allowance = self._get_allowance(caller, operator, Some(id)) + value;

        self.get_mut()
            .operator_approvals
            .insert(&(caller, operator, Some(id)), &new_allowance);

        Ok(())
    }

    fn _decrease_allowance(&mut self, operator: AccountId, id: Id, value: Balance) -> Result<(), PSP35Error> {
        let caller = Self::env().caller();

        if caller == operator {
            return Err(PSP35Error::NotAllowed)
        }

        let initial_allowance = self._get_allowance(caller, operator, Some(id));

        if initial_allowance < value {
            return Err(PSP35Error::InsufficientBalance)
        }

        self.get_mut()
            .operator_approvals
            .insert(&(caller, operator, Some(id)), &(initial_allowance - value));

        Ok(())
    }

    fn _transfer_token(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error> {
        let ids_amounts = vec![(id, value)];
        let operator = Self::env().caller();

        self._decrease_sender_balance(from, id, value)?;
        self._do_safe_transfer_check(&operator, &from, &to, &ids_amounts, &data)?;
        self._increase_receiver_balance(to, id, value);

        Ok(())
    }

    default fn _do_safe_transfer_check(
        &mut self,
        operator: &AccountId,
        from: &AccountId,
        to: &AccountId,
        ids_amounts: &Vec<(Id, Balance)>,
        data: &Vec<u8>,
    ) -> Result<(), PSP35Error> {
        self.flush();
        let builder = PSP35ReceiverRef::before_received_builder(
            to,
            operator.clone(),
            from.clone(),
            ids_amounts.clone(),
            data.clone(),
        )
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
                        Err(PSP35Error::SafeTransferCheckFailed(String::from(
                            "Error during call to receiver",
                        )))
                    }
                }
            }
        };
        self.load();
        result
    }
}

pub trait PSP35Receiver {
    fn before_received(
        &mut self,
        _operator: AccountId,
        _from: AccountId,
        _ids_amounts: Vec<(Id, Balance)>,
        _data: Vec<u8>,
    ) -> Result<(), PSP35Error>;
}

impl<T> PSP35Receiver for T {
    fn before_received(
        &mut self,
        _operator: AccountId,
        _from: AccountId,
        _ids_amounts: Vec<(Id, Balance)>,
        _data: Vec<u8>,
    ) -> Result<(), PSP35Error> {
        Ok(())
    }
}
