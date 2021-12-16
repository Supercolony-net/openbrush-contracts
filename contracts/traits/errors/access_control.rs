/// The AccessControl error type. Contract will throw one of this errors.
/// Modifier returns `MissingRole`.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AccessControlError {
    InvalidCaller,
    MissingRole,
    RoleRedundant,
}