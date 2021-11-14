use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        InkStorage,
    },
};
pub use common::errors::{
    PSP1155Error,
    PSP1155ReceiverError,
};
use core::result::Result;
use ink_env::Error as EnvError;
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

#[brush::wrapper]
pub type PSP1155Wrapper = dyn PSP1155;

/// Contract module which provides a basic implementation of multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
///
/// This module is used through embedding of `PSP1155Data` and implementation of `PSP1155` and
/// `PSP1155Storage` traits.
#[brush::trait_definition]
pub trait PSP1155: PSP1155Storage {
    /// Returns the amount of tokens of token type `id` owned by `account`.
    #[ink(message)]
    fn balance_of(&self, account: AccountId, id: Id) -> Balance {
        self._balance_of_or_zero(account, id)
    }

    /// Batched version of {balance_of}.
    #[ink(message)]
    fn balance_of_batch(&self, accounts_ids: Vec<(AccountId, Id)>) -> Vec<Balance> {
        let values: Vec<Balance> = accounts_ids
            .into_iter()
            .map(|item| self._balance_of_or_zero(item.0, item.1))
            .collect();
        values
    }

    /// Grants or revokes permission to `operator` to transfer the caller's tokens, according to `approved`
    ///
    /// On success a `ApprovalForAll` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP1155Error> {
        let caller = Self::env().caller();
        if caller == operator {
            return Err(PSP1155Error::NotAllowed)
        }
        self.get_mut().operator_approval.insert((caller, operator), approved);

        self._emit_approval_for_all_event(caller, operator, approved);
        Ok(())
    }

    /// Returns true if `operator` is approved to transfer ``account``'s tokens.
    #[ink(message)]
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
        self._is_approved_for_all(account, operator)
    }

    /// Transfers `amount` tokens of token type `id` from `from` to `to`. Also some `data` can be passed.
    ///
    /// On success a `TransferSingle` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TransferToZeroAddress` error if recipient is zero account.
    ///
    /// Returns `NotAllowed` error if transfer is not approved.
    ///
    /// Returns `InsufficientBalance` error if `from` doesn't contain enough balance.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer_from(
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
        self._before_token_transfer(&from, &to, &ids_amounts)?;
        self._do_safe_transfer_check(operator, from, to, ids_amounts, data)?;

        self._transfer_from(from, to, id, amount)?;

        self._emit_transfer_single_event(operator, Some(from), Some(to), id, amount);
        Ok(())
    }

    /// Batched version of {safe_transfer_from}.
    ///
    /// On success a `TransferBatch` event is emitted.
    #[ink(message)]
    fn batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();
        self._transfer_guard(operator, from, to)?;

        self._before_token_transfer(&from, &to, &ids_amounts)?;
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

        self._emit_transfer_batch_event(operator, Some(from), Some(to), ids_amounts);
        Ok(())
    }

    // Helper functions

    fn _emit_transfer_single_event(
        &self,
        _operator: AccountId,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _id: Id,
        _amount: Balance,
    ) {
    }

    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    fn _emit_transfer_batch_event(
        &self,
        _operator: AccountId,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _ids_to_amounts: Vec<(Id, Balance)>,
    ) {
    }

    /// Creates `amount` tokens of token type `id` to `to`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `TransferToZeroAddress` error if `to` is zero account.
    fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();
        if to.is_zero() {
            return Err(PSP1155Error::TransferToZeroAddress)
        }

        if ids_amounts.is_empty() {
            return Ok(())
        }

        for (id, amount) in ids_amounts.iter() {
            self._increase_receiver_balance(to, id.clone(), amount.clone());
        }

        if ids_amounts.len() == 1 {
            self._emit_transfer_single_event(operator, None, Some(to), ids_amounts[0].0, ids_amounts[0].1);
        } else {
            self._emit_transfer_batch_event(operator, None, Some(to), ids_amounts);
        }
        Ok(())
    }

    /// Destroys `amount` tokens of token type `id` from `from`.
    ///
    /// On success a `TransferSingle` event is emitted if length of `ids_amounts` is 1, otherwise `TransferBatch` event.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if transfer is not approved.
    ///
    /// Returns with `InsufficientBalance` error if `from` doesn't contain enough balance.
    fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
        let operator = Self::env().caller();
        if from != operator && !self._is_approved_for_all(from, operator) {
            return Err(PSP1155Error::NotAllowed)
        }

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
            self._emit_transfer_batch_event(operator, Some(from), None, ids_amounts);
        }
        Ok(())
    }

    fn _transfer_guard(&self, operator: AccountId, from: AccountId, to: AccountId) -> Result<(), PSP1155Error> {
        if to.is_zero() {
            return Err(PSP1155Error::TransferToZeroAddress)
        }

        if from != operator && !self._is_approved_for_all(from, operator) {
            return Err(PSP1155Error::NotAllowed)
        }
        Ok(())
    }

    fn _transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
        self._decrease_sender_balance(from, id, amount)?;
        self._increase_receiver_balance(to, id, amount);
        Ok(())
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
        let to_balance = self.get_mut().balances.get(&(id, to)).cloned().unwrap_or(0);
        self.get_mut().balances.insert((id, to), to_balance + amount);
    }

    fn _decrease_sender_balance(&mut self, from: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
        let balance = self.balance_of(from, id);
        if balance < amount {
            return Err(PSP1155Error::InsufficientBalance)
        }

        self.get_mut().balances.insert((id, from), balance - amount);
        Ok(())
    }

    fn _before_token_transfer(
        &self,
        _from: &AccountId,
        _to: &AccountId,
        _ids: &Vec<(Id, Balance)>,
    ) -> Result<(), PSP1155Error> {
        Ok(())
    }

    fn _do_safe_transfer_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error> {
        match PSP1155ReceiverWrapper::before_received_builder(&to, operator, from, ids_amounts, data).fire() {
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
        }
    }
}

#[brush::wrapper]
pub type PSP1155ReceiverWrapper = dyn PSP1155Receiver;

/// PSP1155Receiver is a trait for any contract that wants to support safe transfers from a PSP1155
/// multi token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[brush::trait_definition]
pub trait PSP1155Receiver {
    /// Ensures that the smart contract allows reception of PSP1155 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer_from`
    /// or `batch_transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP1155ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        ids_to_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155ReceiverError>;
}
