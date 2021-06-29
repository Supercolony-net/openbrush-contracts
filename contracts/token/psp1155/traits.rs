pub use core::result::Result;
pub use crate::stub::{PSP1155Receiver};
pub use ink_env::{
    call::{FromAccountId},
    Error as Env_error,
};
pub use ink_lang::{ForwardCallMut, Env, StaticEnv};
pub use ink_prelude::string::String;
pub use ink_prelude::{vec::Vec, vec};
pub use ink_storage::{
    collections::HashMap as StorageHashMap,
};
use brush::{
    traits::{InkStorage, AccountId, Balance},
};
pub use psp1155_derive::{PSP1155Storage, PSP1155MetadataStorage};

pub type Id = [u8; 32];

#[brush::storage_trait]
pub trait PSP1155MetadataStorage: InkStorage {
    fn _uri(&self) -> & Option<String>;
    fn _uri_mut(&mut self) -> &mut Option<String>;
}

#[brush::storage_trait]
pub trait PSP1155Storage: InkStorage {
    fn _balances(&self) -> & StorageHashMap<(Id, AccountId), Balance>;
    fn _balances_mut(&mut self) -> &mut StorageHashMap<(Id, AccountId), Balance>;

    fn _operator_approval(&self) -> & StorageHashMap<(AccountId, AccountId), bool>;
    fn _operator_approval_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), bool>;
}

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

/// A standard trait for contracts that manage multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
#[brush::trait_definition]
pub trait IPSP1155 {
    /// Returns the amount of tokens of token type `_id` owned by `_account`.
    #[ink(message)]
    fn balance_of(&self, _account: AccountId, _id: Id) -> Balance {
        self._balance_of_or_zero(_account, _id)
    }

    /// Batched version of {balance_of}.
    #[ink(message)]
    fn balance_of_batch(&self, _accounts: Vec<AccountId>, _ids: Vec<Id>) -> Vec<Balance> {
        assert_eq!(_accounts.len(), _ids.len(), "{}", PSP1155Error::InputLengthMismatch.as_ref());

        let values: Vec<Balance> = _accounts
            .iter()
            .zip(_ids.iter())
            .map(|(account, id)| self._balance_of_or_zero(account.clone(), id.clone()))
            .collect();
        values
    }

    /// Grants or revokes permission to `_operator` to transfer the caller's tokens, according to `_approved`
    #[ink(message)]
    fn set_approval_for_all(&mut self, _operator: AccountId, _approved: bool) {
        let caller = Self::env().caller();
        assert_ne!(caller, _operator, "{}", PSP1155Error::SelfApproval.as_ref());
        *self
            ._operator_approval_mut()
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
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    ) {
        self._transfer_guard(_from, _to);
        self._before_token_transfer(&vec![_id]);
        self._transfer_from(_from, _to, _id, _amount);

        self._do_safe_transfer_acceptance_check(
            Self::env().caller(),
            _from,
            _to,
            _id,
            _amount,
            _data
        );

        self._emit_transfer_single_event(
            Self::env().caller(), _from, _to, _id, _amount);
    }

    /// Batched version of {safe_transfer_from}.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    ) {
        assert_eq!(_ids.len(), _amounts.len(), "{}", PSP1155Error::InputLengthMismatch.as_ref());
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

        self._emit_transfer_batch_event(
            Self::env().caller(), _from, _to, _ids, _amounts);
    }

    // Helper functions

    fn _emit_transfer_single_event(&self,
                                  _operator: AccountId, _from: AccountId,
                                  _to: AccountId, _id: Id, _amount: Balance) {}

    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    fn _emit_transfer_batch_event(&self,
                                 _operator: AccountId, _from: AccountId,
                                 _to: AccountId, _ids: Vec<Id>, _amounts: Vec<Balance>) {}

    fn _mint(&mut self, to: AccountId, id: Id, amount: Balance) {
        let operator = Self::env().caller();

        assert_ne!(to, [0; 32].into(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id]);
        self._increase_receiver_balance(to, id, amount);

        self._do_safe_transfer_acceptance_check(
            operator,
            [0; 32].into(),
            to,
            id,
            amount,
            Vec::new(),
        );

        self._emit_transfer_single_event(
            operator, [0; 32].into(), to, id, amount);
    }

    fn _burn(&mut self, from: AccountId, id: Id, amount: Balance) {
        assert_ne!(from, [0; 32].into(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        self._before_token_transfer(&vec![id]);
        self._decrease_sender_balance(from, id, amount);

        self._emit_transfer_single_event(
            Self::env().caller(), from, [0; 32].into(), id, amount);
    }

    fn _transfer_guard(&self, from: AccountId, to: AccountId) {
        assert_ne!(to, [0; 32].into(), "{}", PSP1155Error::TransferToZeroAddress.as_ref());

        let operator = Self::env().caller();

        if (from != operator) && (!self._is_approved_for_all(from, operator)) {
            panic!("{}", PSP1155Error::ApproveRequired.as_ref());
        }
    }

    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) {
        self._decrease_sender_balance(from, id, amount);
        self._increase_receiver_balance(to, id, amount);
    }

    fn _balance_of_or_zero(&self, owner: AccountId, id: Id) -> Balance {
        self._balances().get(&(id, owner)).cloned().unwrap_or(0)
    }

    fn _is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool {
        self._operator_approval().get(&(_account, _operator)).cloned().unwrap_or(false)
    }

    fn _increase_receiver_balance(
        &mut self,
        to: AccountId,
        id: Id,
        amount: Balance,
    ) {
        let to_balance = self._balances_mut().entry((id, to)).or_insert(0);
        match to_balance.checked_add(amount) {
            Some(new_to_balance) => *to_balance = new_to_balance,
            _ => panic!("{}", PSP1155Error::MaxBalance.as_ref()),
        }
    }

    fn _decrease_sender_balance(
        &mut self,
        from: AccountId,
        id: Id,
        amount: Balance,
    ) {
        match self
            ._balances()
            .get(&(id, from))
            .map(|old_from_balance| old_from_balance.checked_sub(amount))
        {
            Some(Some(new_from_balance)) => self._balances_mut().insert((id, from), new_from_balance),
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
        let mut receiver : PSP1155Receiver = FromAccountId::from_account_id(_to);
        match receiver.call_mut().on_psp1155_received(_operator, _from, _id, _amount, _data).fire()
        {
            Ok(result) => match result {
                Ok(_) => (),
                _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
            },
            Err(e) => match e {
                Env_error::NotCallable => (),
                _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
            },
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
        let mut receiver : PSP1155Receiver = FromAccountId::from_account_id(_to);
        match receiver.call_mut().on_psp1155_batch_received(_operator, _from, _ids, _amounts, _data).fire()
        {
            Ok(result) => match result {
                Ok(_) => (),
                _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
            },
            Err(e) => match e {
                Env_error::NotCallable => (),
                _ => panic!("{}", PSP1155Error::CallFailed.as_ref()),
            },
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
        self._uri().clone()
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
    fn on_psp1155_received(&mut self, _operator: AccountId, _from: AccountId,
                           _id: Id, _value: Balance, _data: Vec<u8>) -> Result<(), PSP1155ReceiverError>;

    /// This function is called at the end of a safe_batch_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp1155_batch_received(&mut self, _operator: AccountId, _from: AccountId,
                                 _ids: Vec<Id>, _values: Vec<Balance>, _data: Vec<u8>) -> Result<(), PSP1155ReceiverError>;
}
