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

pub use crate::traits::{
    errors::{
        PSP37Error,
        PSP37ReceiverError,
    },
    types::Id,
};
use ink::prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type PSP37Ref = dyn PSP37;

/// Contract module which provides a basic implementation of multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
#[openbrush::trait_definition]
pub trait PSP37 {
    /// Returns the amount of tokens of token type `id` owned by `account`.
    ///
    /// If `id` is `None` returns the total number of `owner`'s tokens.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance;

    /// Returns the total amount of token type `id` in the supply.
    ///
    /// If `id` is `None` returns the total number of tokens.
    #[ink(message)]
    fn total_supply(&self, id: Option<Id>) -> Balance;

    /// Returns amount of `id` token of `owner` that `operator` can withdraw
    /// If `id` is `None` returns allowance `Balance::MAX` of all tokens of `owner`
    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance;

    /// Allows `operator` to withdraw the `id` token from the caller's account
    /// multiple times, up to the `value` amount.
    /// If this function is called again it overwrites the current allowance with `value`
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// An `Approval` event is emitted.
    #[ink(message)]
    fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error>;

    /// Transfers `value` of `id` token from `caller` to `to`
    ///
    /// On success a `TransferSingle` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TransferToZeroAddress` error if recipient is zero account.
    ///
    /// Returns `NotAllowed` error if transfer is not approved.
    ///
    /// Returns `InsufficientBalance` error if `caller` doesn't contain enough balance.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP37Error>;

    /// Transfers `amount` tokens of token type `id` from `from` to `to`. Also some `data` can be passed.
    ///
    /// On success a `TransferSingle` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TransferToZeroAddress` error if recipient is zero account.
    ///
    /// Returns `NotAllowed` error if transfer is not approved.
    ///
    /// Returns `InsufficientBalance` error if `from` doesn't contain enough balance.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP37Error>;
}
