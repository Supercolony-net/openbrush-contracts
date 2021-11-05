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
            PausableError::NoPaused => PSP22Error::Custom(String::from("P::NoPaused")),
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

impl Into<PSP22Error> for PSP22ReceiverError {
    fn into(self) -> PSP22Error {
        match self {
            PSP22ReceiverError::TransferRejected(message) => PSP22Error::Custom(message),
        }
    }
}
