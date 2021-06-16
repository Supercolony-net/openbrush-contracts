use brush::traits::{AccountId};

pub type RoleType = u32;

#[derive(strum_macros::AsRefStr)]
pub enum AccessControlError {
    InvalidCaller,
    MissingRole,
}

// TODO: Add comments
#[brush::trait_definition]
pub trait IAccessControl {
    #[ink(message)]
    fn has_role(&self, role: RoleType, address: AccountId) -> bool;

    // TODO: Add get role member count
    // TODO: Add get role member

    #[ink(message)]
    fn get_role_admin(&self, role: RoleType) -> RoleType;

    #[ink(message)]
    fn grant_role(&mut self, role: RoleType, address: AccountId);

    #[ink(message)]
    fn revoke_role(&mut self, role: RoleType, address: AccountId);

    #[ink(message)]
    fn renounce_role(&mut self, role: RoleType, address: AccountId);
}