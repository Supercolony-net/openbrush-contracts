use ink_prelude::string::String;
use psp22::traits::PSP22Error;

/// Enum of errors raised by our lending smart contract
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LendingError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// This error will be thrown when the lender does not have enough allowance
    /// to transfer the lending asset to the contract
    InsufficientAllowanceToLend,
    /// This error will be thrown when the lender tries to lend more amount of asset that they actually own
    InsufficientBalanceToLend,
    /// This error will be thrown if the user tries to lend or borrow asset which is not supported by the lending contract
    /// or if a user tries to use an usupported asset as a collateral
    AssetNotSupported,
}

impl From<PSP22Error> for LendingError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => LendingError::Custom(message),
            PSP22Error::InsufficientBalance => LendingError::Custom(String::from("PSP22::InsufficientBalance")),
            PSP22Error::InsufficientAllowance => LendingError::Custom(String::from("PSP22::InsufficientAllowance")),
            PSP22Error::ZeroRecipientAddress => LendingError::Custom(String::from("PSP22::ZeroRecipientAddress")),
            PSP22Error::ZeroSenderAddress => LendingError::Custom(String::from("PSP22::ZeroSenderAddress")),
            PSP22Error::SafeTransferCheckFailed(message) => LendingError::Custom(message),
        }
    }
}
