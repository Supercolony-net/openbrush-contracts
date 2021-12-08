pub use crate::{
    psp22::*,
    traits::psp22::extensions::wrapper::*,
};
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
    },
};
pub use derive::PSP22WrapperStorage;
use ink_prelude::vec::Vec;
use ink_storage::traits::SpreadLayout;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22WrapperData {
    pub underlying: AccountId,
}

declare_storage_trait!(PSP22WrapperStorage, PSP22WrapperData);

impl<T: PSP22 + PSP22WrapperStorage + PSP22Internal> PSP22Wrapper for T {
    default fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._deposit(amount)?;
        self._mint(account, amount)
    }

    default fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn(Self::env().caller(), amount)?;
        self._withdraw(account, amount)
    }
}

pub trait PSP22WrapperInternal {
    /// Mint wrapped token to cover any underlyingTokens that would have been transfered by mistake. Internal
    /// function that can be exposed with access control if desired.
    fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error>;

    /// helper function to transfer the underlying token from caller to the contract
    fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error>;

    /// helper function to transfer the underlying token
    fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    /// helper function to get balance of underlying tokens in the contract
    fn _underlying_balance(&mut self) -> Balance;

    /// Initalize the wrapper token with defining the underlying PSP22 token
    ///
    /// `underlying` is the token to be wrapped
    fn _init(&mut self, underlying: AccountId);

    /// Getter for caller to `PSP22Wrapper` of `underlying`
    fn _underlying(&mut self) -> &mut PSP22Ref;
}

impl<T: PSP22 + PSP22Internal + PSP22WrapperStorage> PSP22WrapperInternal for T {
    default fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
        let value = self._underlying_balance() - self.total_supply();
        self._mint(account, value)?;
        Ok(value)
    }

    default fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self._underlying()
            .transfer_from(Self::env().caller(), Self::env().account_id(), amount, Vec::<u8>::new())
    }

    default fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._underlying().transfer(account, amount, Vec::<u8>::new())
    }

    default fn _underlying_balance(&mut self) -> Balance {
        self._underlying().balance_of(Self::env().account_id())
    }

    default fn _init(&mut self, underlying: AccountId) {
        PSP22WrapperStorage::get_mut(self).underlying = underlying;
    }

    default fn _underlying(&mut self) -> &mut PSP22Ref {
        &mut PSP22WrapperStorage::get_mut(self).underlying
    }
}
