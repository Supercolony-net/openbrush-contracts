use super::OwnableError;

/// The Diamond error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DiamondError {
    OwnableError(OwnableError),
    FunctionDoesNotExist,
    ImmutableFunction,
    ReplaceExisting,
}

impl From<OwnableError> for DiamondError {
    fn from(error: OwnableError) -> Self {
        DiamondError::OwnableError(error)
    }
}
