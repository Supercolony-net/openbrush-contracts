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
}
