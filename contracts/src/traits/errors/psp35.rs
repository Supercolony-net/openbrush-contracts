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

/// The PSP35 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP35Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the account doesn't contain enough funds.
    InsufficientBalance,
    /// Returned if recipient is zero account.
    TransferToZeroAddress,
    /// Returned if token doesn't exist
    TokenNotExists,
    /// Returned if the caller is not allowed.
    NotAllowed,
    /// Returned if caller trying to approve himself
    SelfApprove,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP35Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP35Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PSP35Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for PSP35Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP35Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP35Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP35Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP35Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP35Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP35Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP35Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP35Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

/// The PSP35Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP35ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

impl From<PSP35ReceiverError> for PSP35Error {
    fn from(error: PSP35ReceiverError) -> Self {
        match error {
            PSP35ReceiverError::TransferRejected(message) => PSP35Error::SafeTransferCheckFailed(message),
        }
    }
}
