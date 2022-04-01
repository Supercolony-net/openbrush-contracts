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

/// The PSP22 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP22Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP22Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PSP22Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for PSP22Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP22Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP22Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP22Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP22Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP22Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP22Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP22Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP22Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22ReceiverError {
    TransferRejected(String),
}

impl From<PSP22ReceiverError> for PSP22Error {
    fn from(error: PSP22ReceiverError) -> Self {
        match error {
            PSP22ReceiverError::TransferRejected(message) => PSP22Error::SafeTransferCheckFailed(message),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22TokenTimelockError {
    PSP22Error(PSP22Error),
    /// Returned if the owner wants to withdraw the tokens before the release time
    CurrentTimeIsBeforeReleaseTime,
    /// Returned if there are no tokens to be released
    NoTokensToRelease,
    /// Returned if the timestamp provided is before the current time
    ReleaseTimeIsBeforeCurrentTime,
}

impl From<PSP22Error> for PSP22TokenTimelockError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => PSP22TokenTimelockError::PSP22Error(PSP22Error::Custom(message)),
            PSP22Error::InsufficientBalance => PSP22TokenTimelockError::PSP22Error(PSP22Error::InsufficientBalance),
            PSP22Error::InsufficientAllowance => PSP22TokenTimelockError::PSP22Error(PSP22Error::InsufficientAllowance),
            PSP22Error::ZeroRecipientAddress => PSP22TokenTimelockError::PSP22Error(PSP22Error::ZeroRecipientAddress),
            PSP22Error::ZeroSenderAddress => PSP22TokenTimelockError::PSP22Error(PSP22Error::ZeroSenderAddress),
            PSP22Error::SafeTransferCheckFailed(message) => {
                PSP22TokenTimelockError::PSP22Error(PSP22Error::SafeTransferCheckFailed(message))
            }
        }
    }
}

impl From<OwnableError> for PSP22TokenTimelockError {
    fn from(ownable: OwnableError) -> Self {
        PSP22TokenTimelockError::PSP22Error(ownable.into())
    }
}

impl From<AccessControlError> for PSP22TokenTimelockError {
    fn from(access: AccessControlError) -> Self {
        PSP22TokenTimelockError::PSP22Error(access.into())
    }
}

impl From<PausableError> for PSP22TokenTimelockError {
    fn from(pausable: PausableError) -> Self {
        PSP22TokenTimelockError::PSP22Error(pausable.into())
    }
}

impl From<ReentrancyGuardError> for PSP22TokenTimelockError {
    fn from(guard: ReentrancyGuardError) -> Self {
        PSP22TokenTimelockError::PSP22Error(guard.into())
    }
}
