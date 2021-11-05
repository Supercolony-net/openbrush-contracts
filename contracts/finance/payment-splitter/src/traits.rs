use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        InkStorage,
    },
};
pub use common::errors::PaymentSplitterError;
use ink_prelude::vec::Vec;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
    Vec as StorageVec,
};
pub use payment_splitter_derive::PaymentSplitterStorage;

use brush::traits::ZERO_ADDRESS;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PaymentSplitterData {
    pub total_shares: Balance,
    pub total_released: Balance,
    pub shares: StorageHashMap<AccountId, Balance>,
    pub released: StorageHashMap<AccountId, Balance>,
    pub payees: StorageVec<AccountId>,
}

declare_storage_trait!(PaymentSplitterStorage, PaymentSplitterData);

/// This contract allows splitting native token payments among a group of accounts. The sender does not need to be aware
/// that the native token will be split in this way, since it is handled transparently by the contract.
///
/// The split can be in equal parts or in any other arbitrary proportion. The way this is specified is by assigning each
/// account to a number of shares. Of all the native tokens that this contract receives, each account will then be able to claim
/// an amount proportional to the percentage of total shares they were assigned.
///
/// `PaymentSplitter` follows a _pull payment_ model. This means that payments are not automatically forwarded to the
/// accounts but kept in this contract, and the actual transfer is triggered as a separate step by calling the `release`
/// function.
///
/// ** Note **: In the substrate balance of contract decreases each block. Because it pays rent for the storage.
/// So during `release`, each next user will get fewer native tokens.
///
/// This module is used through embedding of `PaymentSplitterData` and implementation of `PaymentSplitter` and
/// `PaymentSplitterStorage` traits.
// TODO: Substrate removed rent, so we need simplify logic in this contract
#[brush::trait_definition]
pub trait PaymentSplitter: PaymentSplitterStorage {
    /// Getter for the total shares held by payees.
    #[ink(message)]
    fn total_shares(&self) -> Balance {
        self.get().total_shares.clone()
    }

    /// Getter for the total amount of native token already released.
    #[ink(message)]
    fn total_released(&self) -> Balance {
        self.get().total_released.clone()
    }

    /// Getter for the amount of shares held by an account.
    #[ink(message)]
    fn shares(&self, account: AccountId) -> Balance {
        self.get().shares.get(&account).cloned().unwrap_or(0)
    }

    /// Getter for the amount of native token already released to a payee.
    #[ink(message)]
    fn released(&self, account: AccountId) -> Balance {
        self.get().released.get(&account).cloned().unwrap_or(0)
    }

    /// Getter for the address of the payee number `index`.
    #[ink(message)]
    fn payee(&self, index: u32) -> AccountId {
        self.get().payees.get(index).cloned().unwrap_or(ZERO_ADDRESS.into())
    }

    /// The native token received will be logged with `PaymentReceived` events.
    /// Note that these events are not fully reliable: a contract can receive a native token
    /// without triggering this function. This only affects the reliability of the events
    /// and not the actual splitting of the native token.
    ///
    /// On success a `PayeeAdded` event is emitted.
    #[ink(message, payable)]
    fn receive(&self) {
        self._emit_payee_added_event(Self::env().caller(), Self::env().transferred_balance())
    }

    /// Triggers a transfer to `account` of the amount of native token they are owed, according to their percentage of the
    /// total shares and their previous withdrawals.
    ///
    /// On success a `PaymentReleased` event is emitted.
    #[ink(message)]
    fn release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
        if !self.get().shares.contains_key(&account) {
            return Err(PaymentSplitterError::AccountHasNoShares)
        }

        let mut current_balance = Self::env().balance();
        let minimum_balance = Self::env().minimum_balance() + Self::env().tombstone_deposit();

        if current_balance < minimum_balance {
            current_balance = 0;
        } else {
            current_balance -= minimum_balance;
        }
        let total_received = current_balance + self.get().total_released;
        let shares = self.get().shares.get(&account).unwrap().clone();
        let total_shares = self.get().total_shares;
        let released = self.get_mut().released.entry(account.clone()).or_default().clone();
        let mut payment = total_received * shares / total_shares;

        if payment < released {
            payment = 0;
        } else {
            payment -= released;
        }

        if payment == 0 {
            return Err(PaymentSplitterError::AccountIsNotDuePayment)
        }

        if payment > current_balance {
            payment = current_balance;
        }

        self.get_mut()
            .released
            .entry(account)
            .and_modify(|r| *r = released + payment);
        self.get_mut().total_released += payment;

        let transfer_result = Self::env().transfer(account.clone(), payment);
        if transfer_result.is_err() {
            return Err(PaymentSplitterError::TransferFailed)
        }
        self._emit_payment_released_event(account, payment);
        Ok(())
    }

    // Helper functions

    /// User must override this method in their contract.
    fn _emit_payee_added_event(&self, _account: AccountId, _shares: Balance) {}

    /// User must override this method in their contract.
    fn _emit_payment_received_event(&self, _from: AccountId, _amount: Balance) {}

    /// User must override this method in their contract.
    fn _emit_payment_released_event(&self, _to: AccountId, _amount: Balance) {}

    /// Inits an instance of `PaymentSplitter` where each account in `payees` is assigned the number of shares at
    /// the matching position in the `shares` array.
    ///
    /// All addresses in `payees` must be non-zero. Both arrays must have the same non-zero length, and there must be no
    /// duplicates in `payees`.
    ///
    /// Emits `PayeeAdded` on each account.
    fn _init(&mut self, payees: Vec<(AccountId, Balance)>) -> Result<(), PaymentSplitterError> {
        if payees.is_empty() {
            return Err(PaymentSplitterError::NoPayees)
        }

        for (payee, share) in payees.into_iter() {
            self._add_payee(payee, share)?;
        }
        Ok(())
    }

    fn _add_payee(&mut self, payee: AccountId, share: Balance) -> Result<(), PaymentSplitterError> {
        if payee.is_zero() {
            return Err(PaymentSplitterError::AccountZeroAddress)
        }
        if share == 0 {
            return Err(PaymentSplitterError::SharesAreZero)
        }
        if self.get().shares.contains_key(&payee) {
            return Err(PaymentSplitterError::AlreadyHasShares)
        }

        self.get_mut().payees.push(payee.clone());
        self.get_mut().shares.insert(payee.clone(), share);
        self.get_mut().total_shares += share;
        self._emit_payee_added_event(payee, share);
        Ok(())
    }
}
