use super::{
    PSP22Error,
    PSP22FlashmintError,
};
use ink_prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashBorrowerError {
    FlashloanRejected(String),
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlashLenderError {
    Custom(String),
    FlashloanRejected(String),
}

impl From<FlashBorrowerError> for FlashLenderError {
    fn from(error: FlashBorrowerError) -> Self {
        match error {
            FlashBorrowerError::FlashloanRejected(message) => FlashLenderError::FlashloanRejected(message),
        }
    }
}

impl From<PSP22FlashmintError> for FlashLenderError {
    fn from(error: PSP22FlashmintError) -> Self {
        match error {
            PSP22FlashmintError::Custom(message) => FlashLenderError::Custom(message),
            PSP22FlashmintError::WrongTokenAddress => {
                FlashLenderError::Custom(String::from("Flashmint: Wrong Token Address"))
            }
            PSP22FlashmintError::AllowanceDoesNotAllowRefund => {
                FlashLenderError::Custom(String::from("Flashmint: Allowance does not allow refund"))
            }
        }
    }
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
