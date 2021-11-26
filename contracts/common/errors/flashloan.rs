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
    Custom(String),
    FlashloanRejected(String),
}

impl From<PSP22Error> for FlashBorrowerError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => FlashBorrowerError::Custom(message),
            PSP22Error::InsufficientBalance => FlashBorrowerError::Custom(String::from("PSP22: Insufficient Balance")),
            PSP22Error::InsufficientAllowance => {
                FlashBorrowerError::Custom(String::from("PSP22: Insufficient Allowance"))
            }
            PSP22Error::ZeroRecipientAddress => {
                FlashBorrowerError::Custom(String::from("PSP22: Zero Recipient Address"))
            }
            PSP22Error::ZeroSenderAddress => FlashBorrowerError::Custom(String::from("PSP22: Zero Sender Address")),
            PSP22Error::SafeTransferCheckFailed(message) => FlashBorrowerError::Custom(message),
        }
    }
}

impl From<FlashLenderError> for FlashBorrowerError {
    fn from(error: FlashLenderError) -> Self {
        match error {
            FlashLenderError::Custom(message) => FlashBorrowerError::Custom(message),
            FlashLenderError::WrongTokenAddress => {
                FlashBorrowerError::Custom(String::from("FlashLender: Wrong Token address"))
            }
            FlashLenderError::AllowanceDoesNotAllowRefund => {
                FlashBorrowerError::Custom(String::from("FlashLender: Allowance does not allow refund"))
            }
            FlashLenderError::FlashloanRejected(message) => FlashBorrowerError::FlashloanRejected(message),
        }
    }
}

impl From<OwnableError> for FlashBorrowerError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => FlashBorrowerError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => FlashBorrowerError::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for FlashBorrowerError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => FlashBorrowerError::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => FlashBorrowerError::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => FlashBorrowerError::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for FlashBorrowerError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => FlashBorrowerError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => FlashBorrowerError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for FlashBorrowerError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => FlashBorrowerError::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

impl From<PSP22ReceiverError> for FlashBorrowerError {
    fn from(error: PSP22ReceiverError) -> Self {
        match error {
            PSP22ReceiverError::TransferRejected(message) => FlashBorrowerError::Custom(message),
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
    FlashloanRejected(String),
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
            FlashBorrowerError::Custom(message) => FlashLenderError::Custom(message),
            FlashBorrowerError::FlashloanRejected(message) => FlashLenderError::FlashloanRejected(message),
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

impl From<PSP22ReceiverError> for FlashLenderError {
    fn from(error: PSP22ReceiverError) -> Self {
        match error {
            PSP22ReceiverError::TransferRejected(message) => FlashLenderError::Custom(message),
        }
    }
}
