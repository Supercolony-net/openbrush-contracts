use ink_prelude::string::String;

/// Enum of errors raised by our lending smart contract
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LendingError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
}