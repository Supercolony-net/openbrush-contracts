/// Extension of [`PSP22`] which allows the beneficiary to extract tokens after given time
use crate::traits::*;

use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        InkStorage,
        Timestamp,
    },
};
use ink_env::call::FromAccountId;
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

#[brush::trait_definition]
pub trait PSP22TokenTimelock: PSP22TokenTimelockStorage + PSP22Receiver {
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
    fn release(&mut self) {
        assert!(
            Self::env().block_timestamp() >= self.get_mut().release_time,
            "{}",
            PSP22Error::Custom("Current time is before release time".to_string()).as_ref()
        );
        let mut psp22: PSP22Stub = FromAccountId::from_account_id(self.get_mut().token_address);
        let amount = psp22.balance_of(Self::env().account_id());
        assert!(
            amount > 0,
            "{}",
            PSP22Error::Custom("No tokens to release".to_string()).as_ref()
        );
        match psp22.transfer(self.beneficiary(), amount, Vec::<u8>::new()) {
            Ok(result) => result,
            Err(e) => panic!("{}", e.as_ref()),
        }
    }

    /// Initializes the contract
    fn init(&mut self, token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) {
        self.get_mut().token_address = token_address;
        self.get_mut().beneficiary = beneficiary;
        self.get_mut().release_time = release_time;
    }
}
