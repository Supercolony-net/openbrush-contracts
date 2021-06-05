use ink_lang as ink;
use utils::traits::{AccountId};

pub type RoleType = u32;

#[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AccessControlError {
    InvalidCaller,
    MissingRole,
}

impl core::fmt::Display for AccessControlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccessControlError: {:?}", self)
    }
}

// TODO: Add comments
#[ink::trait_definition]
pub trait IAccessControl {
    #[ink(message)]
    fn has_role(&self, role: RoleType, address: AccountId) -> bool;

    // TODO: Add get role member count
    // TODO: Add get role member

    #[ink(message)]
    fn get_role_admin(&self, role: RoleType) -> RoleType;

    #[ink(message)]
    fn grant_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError>;

    #[ink(message)]
    fn revoke_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError>;

    #[ink(message)]
    fn renounce_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError>;
}