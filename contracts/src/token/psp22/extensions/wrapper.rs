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
    psp22,
    psp22::extensions::wrapper,
    traits::psp22::{
        extensions::wrapper::*,
        *,
    },
};
use ink::{
    env::CallFlags,
    prelude::vec::Vec,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};
pub use psp22::Internal as _;
pub use wrapper::Internal as _;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub underlying: AccountId,
    pub _reserved: Option<()>,
}

impl<T: Storage<psp22::Data> + Storage<Data>> PSP22Wrapper for T {
    default fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._deposit(amount)?;
        self._mint_to(account, amount)
    }

    default fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(Self::env().caller(), amount)?;
        self._withdraw(account, amount)
    }
}

pub trait Internal {
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

impl<T: Storage<psp22::Data> + Storage<Data>> Internal for T {
    default fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
        let value = self._underlying_balance() - self.total_supply();
        self._mint_to(account, value)?;
        Ok(value)
    }

    default fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self._underlying()
            .transfer_from_builder(Self::env().caller(), Self::env().account_id(), amount, Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()
            .unwrap()
    }

    default fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._underlying()
            .transfer_builder(account, amount, Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()
            .unwrap()
    }

    default fn _underlying_balance(&mut self) -> Balance {
        self._underlying().balance_of(Self::env().account_id())
    }

    default fn _init(&mut self, underlying: AccountId) {
        self.data::<Data>().underlying = underlying;
    }

    default fn _underlying(&mut self) -> &mut PSP22Ref {
        &mut self.data::<Data>().underlying
    }
}
