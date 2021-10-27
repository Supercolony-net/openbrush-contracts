use crate::stub::PSP1155Receiver;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        InkStorage,
        ZERO_ADDRESS,
    },
};
use core::result::Result;
use ink_env::{
    call::FromAccountId,
    Error as Env_error,
};
use ink_lang::ForwardCallMut;
use ink_prelude::{
    string::String,
    vec,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use psp1155_derive::PSP1155Storage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

pub type Id = [u8; 32];

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP1155Data {
    pub balances: StorageHashMap<(Id, AccountId), Balance>,
    pub operator_approval: StorageHashMap<(AccountId, AccountId), bool>,
}

declare_storage_trait!(PSP1155Storage, PSP1155Data);

/// The PSP1155 error type. Contract will throw one of this errors.
#[derive(strum_macros::AsRefStr, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP1155Error {
    /// Returned if safe transfer check fails.
    CallFailed,
    /// Returned if the account doesn't contain enough funds.
    InsufficientBalance,
    /// Returned if recipient is zero account.
    TransferToZeroAddress,
    /// Returned if the caller is not allowed.
    NotAllowed,
}

/// Contract module which provides a basic implementation of multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
///
/// This module is used through embedding of `PSP1155Data` and implementation of `PSP1155` and
/// `PSP1155Storage` traits.
#[brush::trait_definition]
pub trait PSP1155: PSP1155Storage {
    /// Returns the amount of tokens of token type `_id` owned by `_account`.
    #[ink(message)]
    fn balance_of(&self, account: AccountId, id: Id) -> Balance {
        self._balance_of_or_zero(account, id)
    }

    /// Batched version of {balance_of}.
    #[ink(message)]
    fn balance_of_batch(&self, _accounts_ids: Vec<(AccountId, Id)>) -> Vec<Balance> {
        let values: Vec<Balance> = _accounts_ids
            .iter()
            .map(|item| self._balance_of_or_zero(item.0.clone(), item.1.clone()))
            .collect();
        values
    }

    /// Grants or revokes permission to `_operator` to transfer the caller's tokens, according to `_approved`
    ///
    /// On success a `ApprovalForAll` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `NotAllowed` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP1155Error> {
        let caller = Self::env().caller();
        assert_ne!(caller, operator, "{}", PSP1155Error::NotAllowed.as_ref());
        self.get_mut()
            .operator_approval
            .entry((Self::env().caller(), operator))
            .and_modify(|b| *b = approved)
            .or_insert(approved);

        self._emit_approval_for_all_event(caller, operator, approved);
        Ok(())
    }

    /// Returns true if `_operator` is approved to transfer ``_account``'s tokens.
    #[ink(message)]
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
        self._is_approved_for_all(account, operator)
    }

    /// Transfers `_amount` tokens of token type `_id` from `_from` to `_to`. Also some `_data` can be passed.
    ///
    /// On success a `TransferSingle` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `TransferToZeroAddress` error if recipient is zero account.
    ///
    /// Panics with `NotAllowed` error if transfer is not approved.
    ///
    /// Panics with `InsufficientBalance` error if `_from` doesn't contain enough balance.
    ///
    /// Panics with `CallFailed` error if `_to` doesn't accept transfer.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        self._transfer_guard(from, to);
        self._before_token_transfer(&vec![id], from, to);
        self._transfer_from(from, to, id, amount);

        self._do_safe_transfer_acceptance_check(Self::env().caller(), from, to, id, amount, data)?;

        self._emit_transfer_single_event(Self::env().caller(), from, to, id, amount);
        Ok(())
    }

    /// Batched version of {safe_transfer_from}.
    ///
    /// On success a `TransferBatch` event is emitted.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        self._transfer_guard(from, to);
        self._before_token_transfer(&ids_amounts.iter().map(|item| item.0.clone()).collect(), from, to);

        for item in ids_amounts.iter() {
            self._transfer_from(from, to, item.0.clone(), item.1.clone());
        }

        let result = self._do_batch_safe_transfer_acceptance_check(
            Self::env().caller(),
            from,
            to,
            ids_amounts
                .iter()
                .map(|item| (item.0.clone(), item.1.clone()))
                .collect(),
            data,
        );

        self._emit_transfer_batch_event(Self::env().caller(), from, to, ids_amounts);
        result
    }

    // Helper functions

    fn _emit_transfer_single_event(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
    ) {
    }

    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    fn _emit_transfer_batch_event(
        &self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _ids_to_amounts: Vec<(Id, Balance)>,
    ) {
    }

    fn _mint(&mut self, to: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();

        assert!(!to.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id], ZERO_ADDRESS.into(), to);
        self._increase_receiver_balance(to, id, amount);

        let result = self._do_safe_transfer_acceptance_check(operator, ZERO_ADDRESS.into(), to, id, amount, Vec::new());

        self._emit_transfer_single_event(operator, ZERO_ADDRESS.into(), to, id, amount);

        result
    }

    /// Destroys `amount` tokens of token type `id`
    ///
    /// `from` must not be zero address
    /// `from` must have at least `amount` tokens of token type `id` on their balance
    fn _burn(&mut self, from: AccountId, id: Id, amount: Balance) {
        assert!(!from.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id], from, ZERO_ADDRESS.into());
        self._decrease_sender_balance(from, id, amount);

        self._emit_transfer_single_event(Self::env().caller(), from, ZERO_ADDRESS.into(), id, amount);
    }

    /// Batch version of [`PSP1155::_burn`]
    ///
    /// `ids` and `amounts` must be the same length
    fn _burn_batch(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) {
        assert!(!from.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        let caller = Self::env().caller();
        self._before_token_transfer(
            &ids_amounts.iter().map(|item| item.0.clone()).collect(),
            from,
            ZERO_ADDRESS.into(),
        );

        for item in ids_amounts.iter() {
            self._decrease_sender_balance(from, item.0, item.1);
        }

        self._emit_transfer_batch_event(caller, from, ZERO_ADDRESS.into(), ids_amounts);
    }

    fn _transfer_guard(&self, from: AccountId, to: AccountId) {
        assert!(!to.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        let operator = Self::env().caller();
        if (from != operator) && (!self._is_approved_for_all(from, operator)) {
            panic!("{}", PSP1155Error::NotAllowed.as_ref());
        }
    }

    fn _transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, amount: Balance) {
        self._decrease_sender_balance(from, id, amount);
        self._increase_receiver_balance(to, id, amount);
    }

    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self.get().balances.get(&(id, owner)).cloned().unwrap_or(0)
    }

    fn _is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
        self.get()
            .operator_approval
            .get(&(account, operator))
            .cloned()
            .unwrap_or(false)
    }

    fn _increase_receiver_balance(&mut self, to: AccountId, id: Id, amount: Balance) {
        self.get_mut()
            .balances
            .entry((id, to))
            .and_modify(|b| *b += amount)
            .or_insert(amount);
    }

    fn _decrease_sender_balance(&mut self, from: AccountId, id: Id, amount: Balance) {
        let balance = self.balance_of(from, id);
        assert!(balance >= amount, "{}", PSP1155Error::InsufficientBalance.as_ref());
        self.get_mut().balances.entry((id, from)).and_modify(|b| *b -= amount);
    }

    fn _before_token_transfer(&self, _ids: &Vec<Id>, _from: AccountId, _to: AccountId) {}

    fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        let mut receiver: PSP1155Receiver = FromAccountId::from_account_id(to);
        match receiver
            .call_mut()
            .on_psp1155_received(operator, from, id, amount, data)
            .fire()
        {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(()),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
            Err(e) => {
                match e {
                    Env_error::NotCallable => Ok(()),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
        }
    }

    fn _do_batch_safe_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        let mut receiver: PSP1155Receiver = FromAccountId::from_account_id(to);
        match receiver
            .call_mut()
            .on_psp1155_batch_received(operator, from, ids_amounts, data)
            .fire()
        {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(()),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
            Err(e) => {
                match e {
                    Env_error::NotCallable => Ok(()),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
        }
    }
}

/// The PSP1155Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP1155ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

/// Handles the receipt of a single PSP-1155 token type.
#[brush::trait_definition]
pub trait IPSP1155Receiver {
    /// This function is called at the end of a safe_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp1155_received(
        &mut self,
        _operator: AccountId,
        _from: AccountId,
        _id: Id,
        _value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP1155ReceiverError>;

    /// This function is called at the end of a safe_batch_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp1155_batch_received(
        &mut self,
        _operator: AccountId,
        _from: AccountId,
        _ids_to_amounts: Vec<(Id, Balance)>,
        _data: Vec<u8>,
    ) -> Result<(), PSP1155ReceiverError>;
}
