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
    payment_splitter,
    traits::payment_splitter::*,
};
pub use payment_splitter::Internal as _;

use ink::{
    prelude::vec::Vec,
    storage::traits::ManualKey,
};
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Storage,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub total_shares: Balance,
    pub total_released: Balance,
    pub shares: Mapping<AccountId, Balance, ManualKey<{ STORAGE_KEY + 1 }>>,
    pub released: Mapping<AccountId, Balance, ManualKey<{ STORAGE_KEY + 2 }>>,
    pub payees: Vec<AccountId>,
    pub _reserved: Option<()>,
}

impl<T: Storage<Data>> PaymentSplitter for T {
    default fn total_shares(&self) -> Balance {
        self.data().total_shares.clone()
    }

    default fn total_released(&self) -> Balance {
        self.data().total_released.clone()
    }

    default fn shares(&self, account: AccountId) -> Balance {
        self.data().shares.get(&account).unwrap_or(0)
    }

    default fn released(&self, account: AccountId) -> Balance {
        self.data().released.get(&account).unwrap_or(0)
    }

    default fn payee(&self, index: u32) -> AccountId {
        self.data()
            .payees
            .get(index as usize)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into())
    }

    default fn receive(&mut self) {
        self._emit_payee_added_event(Self::env().caller(), Self::env().transferred_value())
    }

    default fn release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
        if !self.data().shares.get(&account).is_some() {
            return Err(PaymentSplitterError::AccountHasNoShares)
        }

        let balance = Self::env().balance();
        let current_balance = balance.checked_sub(Self::env().minimum_balance()).unwrap_or_default();
        let total_received = current_balance + self.data().total_released;
        let shares = self.data().shares.get(&account).unwrap().clone();
        let total_shares = self.data().total_shares;
        let released = self.data().released.get(&account).unwrap_or_default();
        let payment = total_received * shares / total_shares - released;

        if payment == 0 {
            return Err(PaymentSplitterError::AccountIsNotDuePayment)
        }

        self.data().released.insert(&account, &(released + payment));
        self.data().total_released += payment;

        let transfer_result = Self::env().transfer(account.clone(), payment);
        if transfer_result.is_err() {
            return Err(PaymentSplitterError::TransferFailed)
        }
        self._emit_payment_released_event(account, payment);
        Ok(())
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_payee_added_event(&self, _account: AccountId, _shares: Balance);
    fn _emit_payment_received_event(&self, _from: AccountId, _amount: Balance);
    fn _emit_payment_released_event(&self, _to: AccountId, _amount: Balance);

    /// Inits an instance of `PaymentSplitter` where each account in `payees` is assigned the number of shares at
    /// the matching position in the `shares` array.
    ///
    /// All addresses in `payees` must be non-zero. Both arrays must have the same non-zero length, and there must be no
    /// duplicates in `payees`.
    ///
    /// Emits `PayeeAdded` on each account.
    fn _init(&mut self, payees_and_shares: Vec<(AccountId, Balance)>) -> Result<(), PaymentSplitterError>;

    fn _add_payee(&mut self, payee: AccountId, share: Balance) -> Result<(), PaymentSplitterError>;

    /// Calls the `release` method for each `AccountId` in the `payees` vec.
    fn _release_all(&mut self) -> Result<(), PaymentSplitterError>;
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_payee_added_event(&self, _account: AccountId, _shares: Balance) {}
    default fn _emit_payment_received_event(&self, _from: AccountId, _amount: Balance) {}
    default fn _emit_payment_released_event(&self, _to: AccountId, _amount: Balance) {}

    default fn _init(&mut self, payees_and_shares: Vec<(AccountId, Balance)>) -> Result<(), PaymentSplitterError> {
        if payees_and_shares.is_empty() {
            return Err(PaymentSplitterError::NoPayees)
        }

        for (payee, share) in payees_and_shares.into_iter() {
            self._add_payee(payee, share)?;
        }
        Ok(())
    }

    default fn _add_payee(&mut self, payee: AccountId, share: Balance) -> Result<(), PaymentSplitterError> {
        if payee.is_zero() {
            return Err(PaymentSplitterError::AccountZeroAddress)
        }
        if share == 0 {
            return Err(PaymentSplitterError::SharesAreZero)
        }
        if self.data().shares.get(&payee).is_some() {
            return Err(PaymentSplitterError::AlreadyHasShares)
        }

        self.data().payees.push(payee.clone());
        self.data().shares.insert(&payee, &share);
        self.data().total_shares += share;
        self._emit_payee_added_event(payee, share);
        Ok(())
    }

    default fn _release_all(&mut self) -> Result<(), PaymentSplitterError> {
        let len = self.data().payees.len();

        for i in 0..len {
            let account = self.data().payees[i];
            self.release(account)?;
        }

        Ok(())
    }
}
