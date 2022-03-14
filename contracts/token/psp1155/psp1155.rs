pub use crate::traits::psp1155::*;
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
pub use derive::PSP1155Storage;
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

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP1155Data");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP1155Data {
    pub balances: Mapping<(Id, AccountId), Balance>,
    pub operator_approval: Mapping<(AccountId, AccountId), bool>,
}

declare_storage_trait!(PSP1155Storage, PSP1155Data);

impl<T: PSP1155Storage + Flush> PSP1155 for T {
    default fn balance_of(&self, account: AccountId, id: Id) -> Balance {
        self._balance_of_or_zero(account, id)
    }

    default fn balance_of_batch(&self, accounts_ids: Vec<(AccountId, Id)>) -> Vec<Balance> {
        let values: Vec<Balance> = accounts_ids
            .into_iter()
            .map(|item| self._balance_of_or_zero(item.0, item.1))
            .collect();
        values
    }

    default fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP1155Error> {
        let caller = Self::env().caller();
        if caller == operator {
            return Err(PSP1155Error::NotAllowed)
        }
        self.get_mut().operator_approval.insert((&caller, &operator), &approved);

        self._emit_approval_for_all_event(caller, operator, approved);
        Ok(())
    }

    default fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
        self._is_approved_for_all(account, operator)
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();
        self._transfer_guard(operator, from, to)?;

        let ids_amounts = vec![(id, amount)];

        self._before_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        self._do_safe_transfer_check(operator, from, to, ids_amounts.clone(), data)?;
        self._transfer_from(from, to, id, amount)?;
        self._emit_transfer_single_event(operator, Some(from), Some(to), id, amount);

        self._after_token_transfer(Some(&from), Some(&to), &ids_amounts)
    }

    default fn batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();
        self._transfer_guard(operator, from, to)?;

        self._before_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        self._do_safe_transfer_check(
            operator,
            from,
            to,
            // TODO: Avoid copy of vector
            ids_amounts.clone(),
            data,
        )?;

        for item in ids_amounts.clone().into_iter() {
            self._transfer_from(from, to, item.0, item.1)?;
        }

        self._emit_transfer_batch_event(operator, Some(from), Some(to), ids_amounts.clone());

        self._after_token_transfer(Some(&from), Some(&to), &ids_amounts)
    }
}

pub trait PSP1155Internal {
    fn _emit_transfer_single_event(
        &self,
        _operator: AccountId,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _id: Id,
        _amount: Balance,
    );

    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool);

    fn _emit_transfer_batch_event(
        &self,
        _operator: AccountId,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_to_amounts: Vec<(Id, Balance)>,
    );

    /// Creates `amount` tokens of token type `id` to `to`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `TransferToZeroAddress` error if `to` is zero account.
    fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error>;

    /// Destroys `amount` tokens of token type `id` from `from`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if transfer is not approved.
    ///
    /// Returns with `InsufficientBalance` error if `from` doesn't contain enough balance.
    fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error>;

    fn _transfer_guard(&self, operator: AccountId, from: AccountId, to: AccountId) -> Result<(), PSP1155Error>;

    fn _transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error>;

    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance;

    fn _is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool;

    fn _increase_receiver_balance(&mut self, to: AccountId, id: Id, amount: Balance);

    fn _decrease_sender_balance(&mut self, from: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error>;

    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP1155Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP1155Error>;

    fn _do_safe_transfer_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error>;
}

impl<T: PSP1155Storage + Flush> PSP1155Internal for T {
    default fn _emit_transfer_single_event(
        &self,
        _operator: AccountId,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _id: Id,
        _amount: Balance,
    ) {
    }

    default fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    default fn _emit_transfer_batch_event(
        &self,
        _operator: AccountId,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_to_amounts: Vec<(Id, Balance)>,
    ) {
    }

    default fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();
        if to.is_zero() {
            return Err(PSP1155Error::TransferToZeroAddress)
        }

        if ids_amounts.is_empty() {
            return Ok(())
        }

        self._before_token_transfer(None, Some(&to), &ids_amounts)?;

        for (id, amount) in ids_amounts.iter() {
            self._increase_receiver_balance(to, id.clone(), amount.clone());
        }

        if ids_amounts.len() == 1 {
            self._emit_transfer_single_event(operator, None, Some(to), ids_amounts[0].0, ids_amounts[0].1);
        } else {
            self._emit_transfer_batch_event(operator, None, Some(to), ids_amounts.clone());
        }

        self._after_token_transfer(None, Some(&to), &ids_amounts)
    }

    default fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        self._before_token_transfer(Some(&from), None, &ids_amounts)?;

        if ids_amounts.is_empty() {
            return Ok(())
        }

        for (id, amount) in ids_amounts.iter() {
            self._decrease_sender_balance(from, id.clone(), amount.clone())?;
        }

        let operator = Self::env().caller();
        if ids_amounts.len() == 1 {
            self._emit_transfer_single_event(operator, Some(from), None, ids_amounts[0].0, ids_amounts[0].1);
        } else {
            self._emit_transfer_batch_event(operator, Some(from), None, ids_amounts.clone());
        }

        self._after_token_transfer(Some(&from), None, &ids_amounts)
    }

    default fn _transfer_guard(&self, operator: AccountId, from: AccountId, to: AccountId) -> Result<(), PSP1155Error> {
        if to.is_zero() {
            return Err(PSP1155Error::TransferToZeroAddress)
        }

        if from != operator && !self._is_approved_for_all(from, operator) {
            return Err(PSP1155Error::NotAllowed)
        }
        Ok(())
    }

    default fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) -> Result<(), PSP1155Error> {
        self._decrease_sender_balance(from, id, amount)?;
        self._increase_receiver_balance(to, id, amount);
        Ok(())
    }

    default fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self.get().balances.get((&id, &owner)).unwrap_or(0)
    }

    default fn _is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
        self.get().operator_approval.get((&account, &operator)).unwrap_or(false)
    }

    default fn _increase_receiver_balance(&mut self, to: AccountId, id: Id, amount: Balance) {
        let to_balance = self.get_mut().balances.get((&id, &to)).unwrap_or(0);
        self.get_mut().balances.insert((&id, &to), &(to_balance + amount));
    }

    default fn _decrease_sender_balance(
        &mut self,
        from: AccountId,
        id: Id,
        amount: Balance,
    ) -> Result<(), PSP1155Error> {
        let balance = self.balance_of(from, id);
        if balance < amount {
            return Err(PSP1155Error::InsufficientBalance)
        }

        self.get_mut().balances.insert((&id, &from), &(balance - amount));
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP1155Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP1155Error> {
        Ok(())
    }

    default fn _do_safe_transfer_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        self.flush();
        let builder = PSP1155ReceiverRef::before_received_builder(&to, operator, from, ids_amounts, data)
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
                        Err(PSP1155Error::SafeTransferCheckFailed(String::from(
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
