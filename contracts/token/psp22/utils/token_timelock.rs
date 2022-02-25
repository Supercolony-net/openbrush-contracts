/// Extension of [`PSP22`] which allows the beneficiary to extract tokens after given time
pub use crate::traits::psp22::utils::token_timelock::*;
use ink_env::CallFlags;

pub use crate::traits::psp22::PSP22Ref;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        Timestamp,
    },
};
pub use derive::PSP22TokenTimelockStorage;
use ink_prelude::vec::Vec;
use ink_storage::traits::{
    SpreadAllocate,
    SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22TokenTimelockData {
    token: AccountId,
    beneficiary: AccountId,
    release_time: Timestamp,
}

declare_storage_trait!(PSP22TokenTimelockStorage, PSP22TokenTimelockData);

impl<T: PSP22TokenTimelockStorage> PSP22TokenTimelock for T {
    /// Returns the token address
    default fn token(&self) -> AccountId {
        self.get().token
    }

    /// Returns the beneficiary of the tokens
    default fn beneficiary(&self) -> AccountId {
        self.get().beneficiary
    }

    /// Returns the timestamp when the tokens are released
    default fn release_time(&self) -> Timestamp {
        self.get().release_time
    }

    /// Transfers the tokens held by timelock to the beneficairy
    default fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
        if Self::env().block_timestamp() < self.get_mut().release_time {
            return Err(PSP22TokenTimelockError::CurrentTimeIsBeforeReleaseTime)
        }
        let amount = self._contract_balance();
        if amount == 0 {
            return Err(PSP22TokenTimelockError::NoTokensToRelease)
        }
        self._withdraw(amount)
    }
}

pub trait PSP22TokenTimelockInternal {
    /// Helper function to withdraw tokens
    fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError>;

    /// Helper function to return balance of the contract
    fn _contract_balance(&mut self) -> Balance;

    /// Initializes the contract
    fn _init(
        &mut self,
        token: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), PSP22TokenTimelockError>;

    /// Getter for caller to `PSP22Ref` of `token`
    fn _token(&mut self) -> &mut PSP22Ref;
}

impl<T: PSP22TokenTimelockStorage> PSP22TokenTimelockInternal for T {
    default fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
        let beneficairy = self.beneficiary();
        self._token()
            .transfer_builder(beneficairy, amount, Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;
        Ok(())
    }

    default fn _contract_balance(&mut self) -> Balance {
        self._token().balance_of(Self::env().account_id())
    }

    default fn _init(
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

    default fn _token(&mut self) -> &mut PSP22Ref {
        &mut self.get_mut().token
    }
}
