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
        PSP34Error,
        PSP34ReceiverError,
    },
    types::Id,
};
use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type PSP34Ref = dyn PSP34;

/// Contract module which provides a basic implementation of non fungible token.
#[openbrush::trait_definition]
pub trait PSP34 {
    /// Returns the collection `Id` of the NFT token.
    ///
    /// This can represents the relationship between tokens/contracts/pallets.
    #[ink(message)]
    fn collection_id(&self) -> Id;

    /// Returns the balance of the owner.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    /// Returns the owner of the token if any.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;

    /// Returns `true` if the operator is approved by the owner to withdraw `id` token.
    /// If `id` is `None`, returns `true` if the operator is approved to withdraw all owner's tokens.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    /// Approves `operator` to withdraw the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error>;

    /// Transfer approved or owned token from caller.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` does not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    /// Returns current NFT total supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;
}

#[openbrush::wrapper]
pub type PSP34ReceiverRef = dyn PSP34Receiver;

/// PSP34Receiver is a trait for any contract that wants to support safe transfers from a PSP34
/// token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[openbrush::trait_definition]
pub trait PSP34Receiver {
    /// Ensures that the smart contract allows reception of PSP34 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer`
    /// or `transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP34ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34ReceiverError>;
}
