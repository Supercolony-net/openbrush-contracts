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

pub use crate::traits::errors::{
    FlashBorrowerError,
    FlashLenderError,
};
use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type FlashLenderRef = dyn FlashLender;

/// TODO remove eip link
/// Flash Lender implementation as proposed in https://eips.ethereum.org/EIPS/eip-3156)
#[openbrush::trait_definition]
pub trait FlashLender {
    /// Call this function in `max_flashloan` function in `impl` block of FlashLender
    /// Maximum amount of `token` available to mint
    /// Bounded by the max value of Balance (u128)
    #[ink(message)]
    fn max_flashloan(&mut self, token: AccountId) -> Balance;

    /// Call this function in `flash_fee` function in `impl` block of FlashLender
    /// Fee for borrowing `amount` of the `token`
    ///
    /// Returns `WrongTokenAddress` error if the `token` account id is not this token
    #[ink(message)]
    fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError>;

    /// Call this function in `flashloan` function in `impl` block of FlashLender
    /// Mints `amount` of `token` to `receiver_account` and performs the flashloan
    /// `amount` is then burned along with the fee for the flashloan
    /// `receiver_account` must implement `FlashBorrower`
    ///
    /// Returns `AllowanceDoesNotAllowRefund` error if the contract does not have
    /// enough allowance to transfer borrowed amount and fees from `receiver_account`
    #[ink(message)]
    fn flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError>;
}

#[openbrush::wrapper]
pub type FlashBorrowerRef = dyn FlashBorrower;

/// TODO remove eip link
/// Flash Borrower implementation as proposed in https://eips.ethereum.org/EIPS/eip-3156)
#[openbrush::trait_definition]
pub trait FlashBorrower {
    #[ink(message)]
    fn on_flashloan(
        &mut self,
        initiator: AccountId,
        token: AccountId,
        amount: Balance,
        fee: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashBorrowerError>;
}
