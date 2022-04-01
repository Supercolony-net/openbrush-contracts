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

use super::{
    AccessControlError,
    OwnableError,
    PausableError,
    ReentrancyGuardError,
};
use ink_prelude::string::String;

/// The PaymentSplitter error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PaymentSplitterError {
    Custom(String),
    NoPayees,
    AccountHasNoShares,
    AccountIsNotDuePayment,
    AccountZeroAddress,
    SharesAreZero,
    AlreadyHasShares,
    TransferFailed,
}

impl From<AccessControlError> for PaymentSplitterError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PaymentSplitterError::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PaymentSplitterError::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PaymentSplitterError::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<OwnableError> for PaymentSplitterError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PaymentSplitterError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PaymentSplitterError::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<PausableError> for PaymentSplitterError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PaymentSplitterError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PaymentSplitterError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PaymentSplitterError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PaymentSplitterError::Custom(String::from("RG::ReentrantCall")),
        }
    }
}
