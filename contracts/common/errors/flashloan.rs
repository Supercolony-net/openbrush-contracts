use super::PSP22FlashmintError;
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
            PSP22FlashmintError::FlashloanRejected(message) => FlashLenderError::Custom(message),
        }
    }
}
