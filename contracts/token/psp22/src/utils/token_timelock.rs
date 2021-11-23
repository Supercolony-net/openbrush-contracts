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
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::traits::SpreadLayout;
pub use psp22_derive::PSP22TokenTimelockStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22TokenTimelockData {
    token_address: AccountId,
    beneficiary: AccountId,
    release_time: Timestamp,
}

declare_storage_trait!(PSP22TokenTimelockStorage, PSP22TokenTimelockData);

#[brush::wrapper]
pub type PSP22TokenTimelockWrapper = dyn PSP22TokenTimelock;

#[brush::trait_definition]
pub trait PSP22TokenTimelock: PSP22TokenTimelockStorage {
    /// Returns the token address
    #[ink(message)]
    fn token(&self) -> AccountId {
        self.get().token_address
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
    fn release(&mut self) -> Result<(), PSP22Error> {
        if Self::env().block_timestamp() < self.get_mut().release_time {
            return Err(PSP22Error::Custom(String::from("Current time is before release time")))
        }
        let amount = self.contract_balance();
        if amount == 0 {
            return Err(PSP22Error::Custom(String::from("No tokens to release")))
        }
        self.withdraw(amount)
    }

    /// Helper function to withdraw tokens
    fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        PSP22Caller::transfer(&self.get().token_address, self.beneficiary(), amount, Vec::<u8>::new())
    }

    /// Helper function to return balance of the contract
    fn contract_balance(&self) -> Balance {
        PSP22Caller::balance_of(&self.get().token_address, Self::env().account_id())
    }

    /// Initializes the contract
    fn init(
        &mut self,
        token_address: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), PSP22Error> {
        if release_time <= Self::env().block_timestamp() {
            return Err(PSP22Error::Custom(String::from("Release time is before current time")))
        }
        self.get_mut().token_address = token_address;
        self.get_mut().beneficiary = beneficiary;
        self.get_mut().release_time = release_time;
        Ok(())
    }
}
