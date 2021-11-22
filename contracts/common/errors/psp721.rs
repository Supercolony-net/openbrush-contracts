use super::{
    AccessControlError,
    OwnableError,
    PausableError,
    ReentrancyGuardError,
};
use ink_prelude::string::String;

/// The PSP721 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP721Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if owner approves self
    SelfApprove,
    /// Returned if the caller doesn't have allowance for transferring.
    NotApproved,
    /// Returned if the owner already own the token.
    TokenExists,
    /// Returned if we want to mint to zero address.
    MintToZeroAddress,
    /// Returned if  the token doesn't exist
    TokenNotExists,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP721Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP721Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PSP721Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for PSP721Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP721Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP721Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP721Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP721Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP721Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP721Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP721Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP721Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

/// The PSP721Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP721ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

impl From<PSP721ReceiverError> for PSP721Error {
    fn from(error: PSP721ReceiverError) -> Self {
        match error {
            PSP721ReceiverError::TransferRejected(message) => PSP721Error::SafeTransferCheckFailed(message),
        }
    }
}
