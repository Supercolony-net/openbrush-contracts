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
    psp22::*,
    traits::psp22::extensions::wrapper::*,
};
pub use derive::PSP22WrapperStorage;
use ink_env::CallFlags;
use ink_prelude::vec::Vec;
use openbrush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
    },
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP22WrapperData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP22WrapperData {
    pub underlying: AccountId,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP22WrapperStorage);

impl<T: PSP22 + PSP22WrapperStorage<Data = PSP22WrapperData> + PSP22Internal> PSP22Wrapper for T {
    default fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._deposit(amount)?;
        self._mint(account, amount)
    }

    default fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(Self::env().caller(), amount)?;
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

impl<T: PSP22 + PSP22Internal + PSP22WrapperStorage<Data = PSP22WrapperData>> PSP22WrapperInternal for T {
    default fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
        let value = self._underlying_balance() - self.total_supply();
        self._mint(account, value)?;
        Ok(value)
    }

    default fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self._underlying()
            .transfer_from_builder(Self::env().caller(), Self::env().account_id(), amount, Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()
    }

    default fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._underlying()
            .transfer_builder(account, amount, Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()
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
