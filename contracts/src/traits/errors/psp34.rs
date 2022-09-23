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
use openbrush::traits::String;

/// The PSP34 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP34Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if owner approves self
    SelfApprove,
    /// Returned if the caller doesn't have allowance for transferring.
    NotApproved,
    /// Returned if the owner already own the token.
    TokenExists,
    /// Returned if  the token doesn't exist
    TokenNotExists,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP34Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP34Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PSP34Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for PSP34Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP34Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP34Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP34Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP34Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP34Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP34Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP34Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP34Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

/// The PSP34Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP34ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

impl From<PSP34ReceiverError> for PSP34Error {
    fn from(error: PSP34ReceiverError) -> Self {
        match error {
            PSP34ReceiverError::TransferRejected(message) => PSP34Error::SafeTransferCheckFailed(message),
        }
    }
}
