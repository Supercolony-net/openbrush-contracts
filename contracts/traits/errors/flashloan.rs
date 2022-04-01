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
    PSP22Error,
    PSP22ReceiverError,
    PausableError,
    ReentrancyGuardError,
};
use ink_prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashBorrowerError {
    FlashloanRejected(String),
}

impl From<OwnableError> for FlashBorrowerError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                FlashBorrowerError::FlashloanRejected(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsZero => FlashBorrowerError::FlashloanRejected(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for FlashBorrowerError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => FlashBorrowerError::FlashloanRejected(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => {
                FlashBorrowerError::FlashloanRejected(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                FlashBorrowerError::FlashloanRejected(String::from("AC::InvalidCaller"))
            }
        }
    }
}

impl From<PausableError> for FlashBorrowerError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => FlashBorrowerError::FlashloanRejected(String::from("P::Paused")),
            PausableError::NotPaused => FlashBorrowerError::FlashloanRejected(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for FlashBorrowerError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                FlashBorrowerError::FlashloanRejected(String::from("RG::ReentrantCall"))
            }
        }
    }
}

impl From<PSP22ReceiverError> for FlashBorrowerError {
    fn from(error: PSP22ReceiverError) -> Self {
        match error {
            PSP22ReceiverError::TransferRejected(message) => FlashBorrowerError::FlashloanRejected(message),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashLenderError {
    Custom(String),
    /// Returned if we our flashlendner does not support lending of this token
    WrongTokenAddress,
    /// Returned if the contract does not have enough allowance to transfer borrowed amount and fees
    AllowanceDoesNotAllowRefund,
    /// Callee contract rejected the flashloan
    BorrowerRejected(String),
}

impl From<PSP22Error> for FlashLenderError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => FlashLenderError::Custom(message),
            PSP22Error::InsufficientBalance => FlashLenderError::Custom(String::from("PSP22: Insufficient Balance")),
            PSP22Error::InsufficientAllowance => {
                FlashLenderError::Custom(String::from("PSP22: Insufficient Allowance"))
            }
            PSP22Error::ZeroRecipientAddress => FlashLenderError::Custom(String::from("PSP22: Zero Recipient Address")),
            PSP22Error::ZeroSenderAddress => FlashLenderError::Custom(String::from("PSP22: Zero Sender Address")),
            PSP22Error::SafeTransferCheckFailed(message) => FlashLenderError::Custom(message),
        }
    }
}

impl From<FlashBorrowerError> for FlashLenderError {
    fn from(error: FlashBorrowerError) -> Self {
        match error {
            FlashBorrowerError::FlashloanRejected(message) => FlashLenderError::BorrowerRejected(message),
        }
    }
}

impl From<OwnableError> for FlashLenderError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => FlashLenderError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => FlashLenderError::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for FlashLenderError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => FlashLenderError::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => FlashLenderError::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => FlashLenderError::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for FlashLenderError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => FlashLenderError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => FlashLenderError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for FlashLenderError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => FlashLenderError::Custom(String::from("RG::ReentrantCall")),
        }
    }
}
