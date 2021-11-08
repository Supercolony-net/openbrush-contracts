use super::{
    AccessControlError,
    OwnableError,
    PausableError,
    ReentrancyGuardError,
};
use ink_prelude::string::String;

/// The PSP1155 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP1155Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the account doesn't contain enough funds.
    InsufficientBalance,
    /// Returned if recipient is zero account.
    TransferToZeroAddress,
    /// Returned if the caller is not allowed.
    NotAllowed,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP1155Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP1155Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PSP1155Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for PSP1155Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP1155Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP1155Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP1155Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP1155Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP1155Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP1155Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP1155Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP1155Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

/// The PSP1155Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP1155ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

impl From<PSP1155ReceiverError> for PSP1155Error {
    fn from(error: PSP1155ReceiverError) -> Self {
        match error {
            PSP1155ReceiverError::TransferRejected(message) => PSP1155Error::SafeTransferCheckFailed(message),
        }
    }
}
