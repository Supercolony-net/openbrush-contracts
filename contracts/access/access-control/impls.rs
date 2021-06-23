use ink_storage::{
    traits::{PackedLayout, SpreadLayout},
    Box,
};
pub use ink_storage::{
    collections::HashMap as StorageHashMap,
};
use brush::{
    traits::{InkStorage, AccountId},
};
use crate::traits::{ AccessControlError, RoleType };
pub use access_controll_macro::AccessControlStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct RoleData {
    pub members: Box<StorageHashMap<AccountId, bool>>,
    pub admin_role: RoleType,
}

impl RoleData {
    pub fn new(admin: AccountId) -> Self {
        let mut instance = Self {
            members: Box::new(StorageHashMap::new()),
            admin_role: DEFAULT_ADMIN_ROLE,
        };
        instance.members.insert(admin, true);
        instance
    }
}

impl Default for RoleData {
    fn default() -> Self {
        Self {
            members: Box::new(StorageHashMap::new()),
            admin_role: DEFAULT_ADMIN_ROLE,
        }
    }
}

#[brush::internal_trait_definition]
pub trait AccessControlStorage: InkStorage {
    // Mapping of roles to role data which contains information about members of role
    fn _roles(&self) -> & StorageHashMap<RoleType, RoleData>;
    fn _roles_mut(&mut self) -> &mut StorageHashMap<RoleType, RoleData>;
}

pub trait AccessControl: AccessControlStorage {
    fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        self._has_role(&role, &address)
    }

    fn get_role_admin(&self, role: RoleType) -> RoleType {
        self._get_role_admin(&role)
    }

    fn grant_role(&mut self, role: RoleType, address: AccountId) {
        self._check_role(&self._get_role_admin(&role), &Self::env().caller());

        if !self._has_role(&role, &address) {
            self._roles_mut()
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(address, true);
            // TODO: Emit event
        }
    }

    fn revoke_role(&mut self, role: RoleType, address: AccountId) {
        let caller = Self::env().caller();
        self._check_role(&self._get_role_admin(&role), &caller);
        self._do_revoke_role(role, address);
    }

    fn renounce_role(&mut self, role: RoleType, address: AccountId) {
        assert_eq!(Self::env().caller(), address, "{}", AccessControlError::InvalidCaller.as_ref());
        self._do_revoke_role(role, address);
    }

    fn set_role_admin(&mut self, role: RoleType, admin_role: RoleType) {
        self._roles_mut()
            .entry(role)
            .or_insert_with(RoleData::default)
            .admin_role = admin_role;
        // TODO: Emit event
    }

    // Internal functions

    fn _init_with_caller(&mut self) {
        let caller = Self::env().caller();
        self._init_with_admin(caller);
    }

    fn _init_with_admin(&mut self, admin: AccountId) {
        self._roles_mut().insert(DEFAULT_ADMIN_ROLE, RoleData::new(admin));
        // TODO: Emit event
    }

    fn _has_role(&self, role: &RoleType, address: &AccountId) -> bool {
        match self._roles().get(role) {
            Some(role_data) => role_data.members.get(address).cloned().unwrap_or(false),
            None => false,
        }
    }

    fn _get_role_admin(&self, role: &RoleType) -> RoleType {
        match self._roles().get(role) {
            Some(role_data) => role_data.admin_role.clone(),
            None => DEFAULT_ADMIN_ROLE,
        }
    }

    fn _do_revoke_role(&mut self, role: RoleType, address: AccountId) {
        if self._has_role(&role, &address) {
            self._roles_mut()
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(address, false);
            // TODO: Emit event
        }
    }

    fn _check_role(&self, role: &RoleType, address: &AccountId) {
        assert!(self._has_role(role, address), "{}", AccessControlError::MissingRole.as_ref())
    }
}