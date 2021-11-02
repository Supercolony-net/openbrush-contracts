/// Extension of [`PSP22`] which supports token wrapping
use crate::traits::*;

use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
    },
};
use ink_env::call::FromAccountId;
use ink_prelude::vec::Vec;
use ink_storage::traits::SpreadLayout;
pub use psp22_derive::PSP22WrapperStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22WrapperData {
    pub underlying: AccountId,
}

declare_storage_trait!(PSP22WrapperStorage, PSP22WrapperData);

#[brush::trait_definition]
pub trait PSP22Wrapper: PSP22WrapperStorage + PSP22 + PSP22Receiver {
    /// Allow a user to deposit `amount` of underlying tokens and mint `amount` of the wrapped tokens to `account`
    #[ink(message)]
    fn deposit_for(&mut self, account: AccountId, amount: Balance) -> bool {
        let mut token: PSP22Stub = FromAccountId::from_account_id(PSP22WrapperStorage::get_mut(self).underlying);
        match token.transfer_from(Self::env().caller(), Self::env().account_id(), amount, Vec::<u8>::new()) {
            Ok(result) => result,
            Err(e) => panic!("{}", e.as_ref()),
        }
        self._mint(account, amount);
        true
    }

    /// Allow a user to burn `amount` of wrapped tokens and withdraw the corresponding number of underlying tokens to `account`
    #[ink(message)]
    fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> bool {
        self._burn(Self::env().caller(), amount);
        let mut token: PSP22Stub = FromAccountId::from_account_id(PSP22WrapperStorage::get_mut(self).underlying);
        match token.transfer(account, amount, Vec::<u8>::new()) {
            Ok(result) => result,
            Err(e) => panic!("{}", e.as_ref()),
        }
        true
    }

    /// Mint wrapped token to cover any underlyingTokens that would have been transfered by mistake. Internal
    /// function that can be exposed with access control if desired.
    fn _recover(&mut self, account: AccountId) -> Balance {
        let token: PSP22Stub = FromAccountId::from_account_id(PSP22WrapperStorage::get_mut(self).underlying);
        let value = token.balance_of(Self::env().account_id()) - self.total_supply();
        self._mint(account, value);
        value
    }

    /// Initalize the wrapper token with defining the underlying PSP22 token
    ///
    /// `underlying` is the token to be wrapped
    fn init(&mut self, underlying: AccountId) {
        PSP22WrapperStorage::get_mut(self).underlying = underlying;
    }
}
