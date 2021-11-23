use super::{
    FlashBorrowerError,
    PSP22Error,
};
use ink_prelude::string::String;

/// The PSP22 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22FlashmintError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if we try to get flash fee of other token (not the one implementing the flashmint)
    WrongTokenAddress,
    /// Returned if the contract does not have enough allowance to transfer borrowed amount and fees
    AllowanceDoesNotAllowRefund,
    FlashloanRejected(String),
}

impl From<PSP22Error> for PSP22FlashmintError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => PSP22FlashmintError::Custom(message),
            PSP22Error::InsufficientBalance => PSP22FlashmintError::Custom(String::from("PSP22: Insufficient Balance")),
            PSP22Error::InsufficientAllowance => {
                PSP22FlashmintError::Custom(String::from("PSP22: Insufficient Allowance"))
            }
            PSP22Error::ZeroRecipientAddress => {
                PSP22FlashmintError::Custom(String::from("PSP22: Zero Recipient Address"))
            }
            PSP22Error::ZeroSenderAddress => PSP22FlashmintError::Custom(String::from("PSP22: Zero Sender Address")),
            PSP22Error::SafeTransferCheckFailed(message) => PSP22FlashmintError::Custom(message),
        }
    }
}

impl From<FlashBorrowerError> for PSP22FlashmintError {
    fn from(error: FlashBorrowerError) -> Self {
        match error {
            FlashBorrowerError::FlashloanRejected(message) => PSP22FlashmintError::FlashloanRejected(message),
        }
    }
}
