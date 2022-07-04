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

/// Extension of [`PSP22`] which allows the beneficiary to extract tokens after given time
pub use crate::{
    psp22,
    psp22::utils::token_timelock,
    traits::psp22::{
        utils::token_timelock::*,
        *,
    },
};
pub use psp22::Internal as _;
pub use token_timelock::Internal as _;

use ink_env::CallFlags;
use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    Timestamp,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct Data {
    token: AccountId,
    beneficiary: AccountId,
    release_time: Timestamp,
}

impl<T: Storage<Data>> PSP22TokenTimelock for T {
    /// Returns the token address
    default fn token(&self) -> AccountId {
        self.data().token
    }

    /// Returns the beneficiary of the tokens
    default fn beneficiary(&self) -> AccountId {
        self.data().beneficiary
    }

    /// Returns the timestamp when the tokens are released
    default fn release_time(&self) -> Timestamp {
        self.data().release_time
    }

    /// Transfers the tokens held by timelock to the beneficairy
    default fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
        if Self::env().block_timestamp() < self.data().release_time {
            return Err(PSP22TokenTimelockError::CurrentTimeIsBeforeReleaseTime)
        }
        let amount = self._contract_balance();
        if amount == 0 {
            return Err(PSP22TokenTimelockError::NoTokensToRelease)
        }
        self._withdraw(amount)
    }
}

pub trait Internal {
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

impl<T: Storage<Data>> Internal for T {
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
        self.data().token = token;
        self.data().beneficiary = beneficiary;
        self.data().release_time = release_time;
        Ok(())
    }

    default fn _token(&mut self) -> &mut PSP22Ref {
        &mut self.data().token
    }
}
