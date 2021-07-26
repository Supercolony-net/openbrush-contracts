use ink_storage::{
    traits::{PackedLayout, SpreadLayout},
};
use ink_prelude::collections::BTreeMap;
use brush::traits::{InkStorage, AccountId};
use brush::declare_storage_trait;
pub use access_control_derive::AccessControlStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct AccessControlData {
    pub roles: BTreeMap<RoleType, RoleData>,
}

declare_storage_trait!(AccessControlStorage, AccessControlData);

pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct RoleData {
    pub members: BTreeMap<AccountId, bool>,
    pub admin_role: RoleType,
}

impl RoleData {
    pub fn new(admin: AccountId) -> Self {
        let mut instance = Self {
            members: BTreeMap::new(),
            admin_role: DEFAULT_ADMIN_ROLE,
        };
        instance.members.insert(admin, true);
        instance
    }
}

impl Default for RoleData {
    fn default() -> Self {
        Self {
            members: BTreeMap::new(),
            admin_role: DEFAULT_ADMIN_ROLE,
        }
    }
}

pub type RoleType = u32;

#[derive(strum_macros::AsRefStr)]
pub enum AccessControlError {
    InvalidCaller,
    MissingRole,
}

// TODO: Add comments
#[brush::trait_definition]
pub trait IAccessControl: AccessControlStorage {
    #[ink(message)]
    fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        self._has_role(&role, &address)
    }

    // TODO: Add get role member count
    // TODO: Add get role member

    #[ink(message)]
    fn get_role_admin(&self, role: RoleType) -> RoleType {
        self._get_role_admin(&role)
    }

    #[ink(message)]
    fn grant_role(&mut self, role: RoleType, address: AccountId) {
        self._check_role(&self._get_role_admin(&role), &Self::env().caller());

        if !self._has_role(&role, &address) {
            self.get_mut().roles
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(address, true);
            self._emit_role_granted(role, address, Some(Self::env().caller()))
        }
    }

    #[ink(message)]
    fn revoke_role(&mut self, role: RoleType, address: AccountId) {
        let caller = Self::env().caller();
        self._check_role(&self._get_role_admin(&role), &caller);
        self._do_revoke_role(role, address);
    }

    #[ink(message)]
    fn renounce_role(&mut self, role: RoleType, address: AccountId) {
        assert_eq!(Self::env().caller(), address, "{}", AccessControlError::InvalidCaller.as_ref());
        self._do_revoke_role(role, address);
    }

    // Helper functions

    /// The user must override this function using their event definition.
    fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous_admin_role: RoleType, _new_admin_role: RoleType) { }

    /// The user must override this function using their event definition.
    fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>) { }

    /// The user must override this function using their event definition.
    fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId) { }

    fn _init_with_caller(&mut self) {
        let caller = Self::env().caller();
        self._init_with_admin(caller);
    }

    fn _init_with_admin(&mut self, admin: AccountId) {
        self.get_mut().roles.insert(DEFAULT_ADMIN_ROLE, RoleData::new(admin));
        self._emit_role_granted(DEFAULT_ADMIN_ROLE, admin, None);
    }

    fn _has_role(&self, role: &RoleType, address: &AccountId) -> bool {
        match self.get().roles.get(role) {
            Some(role_data) => role_data.members.get(address).cloned().unwrap_or(false),
            None => false,
        }
    }

    fn _get_role_admin(&self, role: &RoleType) -> RoleType {
        match self.get().roles.get(role) {
            Some(role_data) => role_data.admin_role.clone(),
            None => DEFAULT_ADMIN_ROLE,
        }
    }

    fn _do_revoke_role(&mut self, role: RoleType, address: AccountId) {
        if self._has_role(&role, &address) {
            self.get_mut().roles
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(address, false);
            self._emit_role_revoked(role, address, Self::env().caller());
        }
    }

    fn _check_role(&self, role: &RoleType, address: &AccountId) {
        assert!(self._has_role(role, address), "{}", AccessControlError::MissingRole.as_ref())
    }

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let old_admin = self.get_mut().roles
            .entry(role)
            .or_insert_with(RoleData::default).admin_role;
        self._emit_role_admin_changed(role, old_admin, new_admin);
        self.get_mut().roles.entry(role).or_insert_with(RoleData::default).admin_role = new_admin;
    }
}
