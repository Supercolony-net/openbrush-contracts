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
    traits::SpreadLayout,
    collections::HashMap as StorageHashMap,
};
pub use psp1155_derive::{
    PSP1155MetadataStorage,
    PSP1155Storage,
};

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

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP1155MetadataData {
    pub uri: Option<String>,
}

declare_storage_trait!(PSP1155MetadataStorage, PSP1155MetadataData);

/// The PSP1155 error type. Contract will throw one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum PSP1155Error {
    Unknown(String),
    CallFailed,
    ZeroAddress,
    SelfApproval,
    InsufficientBalance,
    MaxBalance,
    TransferToZeroAddress,
    ApproveRequired,
    InputLengthMismatch,
}

/// Contract module which provides a basic implementation of multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
///
/// This module is used through embedding of `PSP1155Data` and implementation of `IPSP1155` and
/// `PSP1155Storage` traits.
#[brush::trait_definition]
pub trait IPSP1155: PSP1155Storage {
    /// Returns the amount of tokens of token type `_id` owned by `_account`.
    #[ink(message)]
    fn balance_of(&self, _account: AccountId, _id: Id) -> Balance {
        self._balance_of_or_zero(_account, _id)
    }

    /// Batched version of {balance_of}.
    #[ink(message)]
    fn balance_of_batch(&self, _accounts: Vec<AccountId>, _ids: Vec<Id>) -> Vec<Balance> {
        assert_eq!(
            _accounts.len(),
            _ids.len(),
            "{}",
            PSP1155Error::InputLengthMismatch.as_ref()
        );

        let values: Vec<Balance> = _accounts
            .iter()
            .zip(_ids.iter())
            .map(|(account, id)| self._balance_of_or_zero(account.clone(), id.clone()))
            .collect();
        values
    }

    /// Grants or revokes permission to `_operator` to transfer the caller's tokens, according to `_approved`
    ///
    /// On success a `ApprovalForAll` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `SelfApproval` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, _operator: AccountId, _approved: bool) {
        let caller = Self::env().caller();
        assert_ne!(caller, _operator, "{}", PSP1155Error::SelfApproval.as_ref());
        *self
            .get_mut()
            .operator_approval
            .entry((Self::env().caller(), _operator))
            .or_insert(false) = _approved;

        self._emit_approval_for_all_event(caller, _operator, _approved);
    }

    /// Returns true if `_operator` is approved to transfer ``_account``'s tokens.
    #[ink(message)]
    fn is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
        self._is_approved_for_all(_account, _operator)
    }

    /// Transfers `_amount` tokens of token type `_id` from `_from` to `_to`. Also some `_data` can be passed.
    ///
    /// On success a `TransferSingle` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `TransferToZeroAddress` error if receipt is zero account.
    ///
    /// Panics with `ApproveRequired` error if transfer is not approved.
    ///
    /// Panics with `InsufficientBalance` error if `_from` doesn't contain enough balance.
    ///
    /// Panics with `CallFailed` error if `_to` doesn't accept transfer.
    #[ink(message)]
    fn safe_transfer_from(&mut self, _from: AccountId, _to: AccountId, _id: Id, _amount: Balance, _data: Vec<u8>) {
        self._transfer_guard(_from, _to);
        self._before_token_transfer(&vec![_id]);
        self._transfer_from(_from, _to, _id, _amount);

        self._do_safe_transfer_acceptance_check(Self::env().caller(), _from, _to, _id, _amount, _data);

        self._emit_transfer_single_event(Self::env().caller(), _from, _to, _id, _amount);
    }

    /// Batched version of {safe_transfer_from}.
    ///
    /// On success a `TransferBatch` event is emitted.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) {
        assert_eq!(
            _ids.len(),
            _amounts.len(),
            "{}",
            PSP1155Error::InputLengthMismatch.as_ref()
        );
        self._transfer_guard(_from, _to);
        self._before_token_transfer(&_ids);

        for (id, value) in _ids.iter().zip(_amounts.iter()) {
            self._transfer_from(_from, _to, id.clone(), value.clone());
        }

        self._do_batch_safe_transfer_acceptance_check(
            Self::env().caller(),
            _from,
            _to,
            _ids.clone(),
            _amounts.clone(),
            _data,
        );

        self._emit_transfer_batch_event(Self::env().caller(), _from, _to, _ids, _amounts);
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
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
    ) {
    }

    fn _mint(&mut self, to: AccountId, id: Id, amount: Balance) {
        let operator = Self::env().caller();

        assert!(!to.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id]);
        self._increase_receiver_balance(to, id, amount);

        self._do_safe_transfer_acceptance_check(operator, ZERO_ADDRESS.into(), to, id, amount, Vec::new());

        self._emit_transfer_single_event(operator, ZERO_ADDRESS.into(), to, id, amount);
    }

    fn _burn(&mut self, from: AccountId, id: Id, amount: Balance) {
        assert!(!from.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id]);
        self._decrease_sender_balance(from, id, amount);

        self._emit_transfer_single_event(Self::env().caller(), from, ZERO_ADDRESS.into(), id, amount);
    }

    fn _transfer_guard(&self, from: AccountId, to: AccountId) {
        assert!(!to.is_zero(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        let operator = Self::env().caller();

        if (from != operator) && (!self._is_approved_for_all(from, operator)) {
            panic!("{}", PSP1155Error::ApproveRequired.as_ref());
        }
    }

    fn _transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, amount: Balance) {
        self._decrease_sender_balance(from, id, amount);
        self._increase_receiver_balance(to, id, amount);
    }

    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self.get().balances.get(&(id, owner)).cloned().unwrap_or(0)
    }

    fn _is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
        self.get()
            .operator_approval
            .get(&(_account, _operator))
            .cloned()
            .unwrap_or(false)
    }

    fn _increase_receiver_balance(&mut self, to: AccountId, id: Id, amount: Balance) {
        let to_balance = self.get_mut().balances.entry((id, to)).or_insert(0);
        match to_balance.checked_add(amount) {
            Some(new_to_balance) => *to_balance = new_to_balance,
            _ => panic!("{}", PSP1155Error::MaxBalance.as_ref()),
        }
    }

    fn _decrease_sender_balance(&mut self, from: AccountId, id: Id, amount: Balance) {
        match self
            .get()
            .balances
            .get(&(id, from))
            .map(|old_from_balance| old_from_balance.checked_sub(amount))
        {
            Some(Some(new_from_balance)) => self.get_mut().balances.insert((id, from), new_from_balance),
            _ => panic!("{}", PSP1155Error::InsufficientBalance.as_ref()),
        };
    }

    fn _before_token_transfer(&self, _ids: &Vec<Id>) {}

    fn _do_safe_transfer_acceptance_check(
        &mut self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) {
        let mut receiver: PSP1155Receiver = FromAccountId::from_account_id(_to);
        match receiver
            .call_mut()
            .on_psp1155_received(_operator, _from, _id, _amount, _data)
            .fire()
        {
            Ok(result) => {
                match result {
                    Ok(_) => (),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
            Err(e) => {
                match e {
                    Env_error::NotCallable => (),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
        }
    }

    fn _do_batch_safe_transfer_acceptance_check(
        &mut self,
        _operator: AccountId,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) {
        let mut receiver: PSP1155Receiver = FromAccountId::from_account_id(_to);
        match receiver
            .call_mut()
            .on_psp1155_batch_received(_operator, _from, _ids, _amounts, _data)
            .fire()
        {
            Ok(result) => {
                match result {
                    Ok(_) => (),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
            Err(e) => {
                match e {
                    Env_error::NotCallable => (),
                    _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
                }
            }
        }
    }
}

#[brush::trait_definition]
pub trait IPSP1155Mint: IPSP1155 {
    /// Mints a new token.
    #[ink(message)]
    fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
        self._mint(to, id, amount)
    }

    /// Burns an existing token.
    #[ink(message)]
    fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
        self._burn(from, id, amount)
    }
}

#[brush::trait_definition]
pub trait IPSP1155Metadata: PSP1155MetadataStorage {
    /// Returns the URI for token type `id`.
    #[ink(message)]
    fn uri(&self, _id: Id) -> Option<String> {
        self.get().uri.clone()
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
        _ids: Vec<Id>,
        _values: Vec<Balance>,
        _data: Vec<u8>,
    ) -> Result<(), PSP1155ReceiverError>;
}
