use super::OwnableError;
use ink_prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ProxyError {
    Custom(String),
    NewImplementationIsZero,
    UnderlyingTransactionReverted,
}

impl From<OwnableError> for ProxyError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => ProxyError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => ProxyError::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}
