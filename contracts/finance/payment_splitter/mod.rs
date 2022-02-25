pub use crate::traits::payment_splitter::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        ZERO_ADDRESS,
    },
};
pub use derive::PaymentSplitterStorage;
use ink_prelude::vec::Vec;
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PaymentSplitterData {
    pub total_shares: Balance,
    pub total_released: Balance,
    pub shares: Mapping<AccountId, Balance>,
    pub released: Mapping<AccountId, Balance>,
    // TODO: change this
    pub payees: Vec<AccountId>,
}

declare_storage_trait!(PaymentSplitterStorage, PaymentSplitterData);

impl<T: PaymentSplitterStorage> PaymentSplitter for T {
    default fn total_shares(&self) -> Balance {
        self.get().total_shares.clone()
    }

    default fn total_released(&self) -> Balance {
        self.get().total_released.clone()
    }

    default fn shares(&self, account: AccountId) -> Balance {
        self.get().shares.get(&account).unwrap_or(0)
    }

    default fn released(&self, account: AccountId) -> Balance {
        self.get().released.get(&account).unwrap_or(0)
    }

    default fn payee(&self, index: u32) -> AccountId {
        self.get()
            .payees
            .get(index as usize)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into())
    }

    default fn receive(&mut self) {
        self._emit_payee_added_event(Self::env().caller(), Self::env().transferred_value())
    }

    default fn release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError> {
        if !self.get().shares.get(account).is_some() {
            return Err(PaymentSplitterError::AccountHasNoShares)
        }

        let current_balance = Self::env()
            .balance()
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        let total_received = current_balance + self.get().total_released;
        let shares = self.get().shares.get(&account).unwrap().clone();
        let total_shares = self.get().total_shares;
        let released = self.get_mut().released.get(&account).unwrap_or_default();
        let payment = total_received * shares / total_shares - released;

        if payment == 0 {
            return Err(PaymentSplitterError::AccountIsNotDuePayment)
        }

        self.get_mut().released.insert(account, &(released + payment));
        self.get_mut().total_released += payment;

        let transfer_result = Self::env().transfer(account.clone(), payment);
        if transfer_result.is_err() {
            return Err(PaymentSplitterError::TransferFailed)
        }
        self._emit_payment_released_event(account, payment);
        Ok(())
    }
}

pub trait PaymentSplitterInternal {
    /// User must override this method in their contract.
    fn _emit_payee_added_event(&self, _account: AccountId, _shares: Balance);

    /// User must override this method in their contract.
    fn _emit_payment_received_event(&self, _from: AccountId, _amount: Balance);

    /// User must override this method in their contract.
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
}

impl<T: PaymentSplitterStorage> PaymentSplitterInternal for T {
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
        if self.get().shares.get(payee).is_some() {
            return Err(PaymentSplitterError::AlreadyHasShares)
        }

        self.get_mut().payees.push(payee.clone());
        self.get_mut().shares.insert(payee.clone(), &share);
        self.get_mut().total_shares += share;
        self._emit_payee_added_event(payee, share);
        Ok(())
    }
}
