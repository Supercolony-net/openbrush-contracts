/// Extension of [`PSP22`] which allows the beneficiary to extract tokens after given time
use crate::traits::*;

use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
        Timestamp,
    },
};
pub use common::errors::PSP22TokenTimelockError;
use ink_prelude::vec::Vec;
use ink_storage::traits::SpreadLayout;
pub use psp22_derive::PSP22TokenTimelockStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22TokenTimelockData {
    token: AccountId,
    beneficiary: AccountId,
    release_time: Timestamp,
}

declare_storage_trait!(PSP22TokenTimelockStorage, PSP22TokenTimelockData);

#[brush::wrapper]
pub type PSP22TokenTimelockRef = dyn PSP22TokenTimelock;

#[brush::trait_definition]
pub trait PSP22TokenTimelock: PSP22TokenTimelockStorage {
    /// Returns the token address
    #[ink(message)]
    fn token(&self) -> AccountId {
        self.get().token
    }

    /// Returns the beneficiary of the tokens
    #[ink(message)]
    fn beneficiary(&self) -> AccountId {
        self.get().beneficiary
    }

    /// Returns the timestamp when the tokens are released
    #[ink(message)]
    fn release_time(&self) -> Timestamp {
        self.get().release_time
    }

    /// Transfers the tokens held by timelock to the beneficairy
    #[ink(message)]
    fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
        if Self::env().block_timestamp() < self.get_mut().release_time {
            return Err(PSP22TokenTimelockError::CurrentTimeIsBeforeReleaseTime)
        }
        let amount = self._contract_balance();
        if amount == 0 {
            return Err(PSP22TokenTimelockError::NoTokensToRelease)
        }
        self._withdraw(amount)
    }

    // Helper functions

    /// Helper function to withdraw tokens
    fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
        let beneficairy = self.beneficiary();
        self._token().transfer(beneficairy, amount, Vec::<u8>::new())?;
        Ok(())
    }

    /// Helper function to return balance of the contract
    fn _contract_balance(&mut self) -> Balance {
        self._token().balance_of(Self::env().account_id())
    }

    /// Initializes the contract
    fn _init(
        &mut self,
        token: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), PSP22TokenTimelockError> {
        if release_time <= Self::env().block_timestamp() {
            return Err(PSP22TokenTimelockError::ReleaseTimeIsBeforeCurrentTime)
        }
        self.get_mut().token = token;
        self.get_mut().beneficiary = beneficiary;
        self.get_mut().release_time = release_time;
        Ok(())
    }

    /// Getter for caller to `PSP22Ref` of `token`
    fn _token(&mut self) -> &mut PSP22Ref {
        &mut self.get_mut().token
    }
}
