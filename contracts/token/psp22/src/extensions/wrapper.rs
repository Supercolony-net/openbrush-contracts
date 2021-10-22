/// Extension of [`PSP22`] which supports token wrapping
use crate::traits::*;

use self::PSP22;
use crate::stub::PSP22 as PSP22Stub;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
    },
};
use ink_storage::traits::SpreadLayout;
pub use psp22_derive::PSP22WrapperStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22WrapperData {
    pub underlying: PSP22Stub,
}

declare_storage_trait!(PSP22WrapperStorage, PSP22WrapperData);

#[brush::trait_definition]
pub trait PSP22Wrapper: PSP22WrapperStorage + PSP22 {
    /// Initalize the wrapper token with defining the underlying PSP22 token
    ///
    /// `underlying` is the token to be wrapped
    #[ink(message)]
    fn init(&mut self, underlying: PSP22Stub) {
        PSP22WrapperStorage::get_mut(self).underlying = underlying;
    }

    /// Allow a user to deposit `amount` of underlying tokens and mint `amount` of the wrapped tokens to `account`
    #[ink(message)]
    fn deposit_for(&mut self, account: AccountId, amount: Balance) -> bool {
        PSP22WrapperStorage::get_mut(self).underlying.transfer_from(
            Self::env().caller(),
            Self::env().account_id(),
            amount,
            Vec::new(),
        );
        self._mint(account, amount);
        true
    }

    /// Allow a user to burn `amount` of wrapped tokens and withdraw the corresponding number of underlying tokens to `account`
    #[ink(message)]
    fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> bool {
        self._burn(Self::env().caller(), amount);
        PSP22WrapperStorage::get_mut(self)
            .underlying
            .transfer(account, amount, Vec::new());
        true
    }

    /// Mint wrapped token to cover any underlyingTokens that would have been transfered by mistake. Internal
    /// function that can be exposed with access control if desired.
    fn _recover(&mut self, account: AccountId) -> Balance {
        let value = PSP22WrapperStorage::get_mut(self)
            .underlying
            .balance_of(Self::env().account_id())
            - self.total_supply();
        self._mint(account, value);
        value
    }
}
