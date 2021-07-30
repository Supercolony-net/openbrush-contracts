pub use access_control_derive::AccessControlStorage;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        InkStorage,
    },
};
use ink_prelude::collections::BTreeMap;
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};

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

/// The AccessControl error type. Contract will throw one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum AccessControlError {
    InvalidCaller,
    MissingRole,
}

/// Contract module that allows children to implement role-based access
/// control mechanisms. This is a lightweight version that doesn't allow enumerating role
/// members except through off-chain means by accessing the contract event logs.
///
/// Roles can be granted and revoked dynamically via the `grant_role` and
/// `revoke_role`. functions. Each role has an associated admin role, and only
/// accounts that have a role's admin role can call `grant_role` and `revoke_role`.
///
/// This module is used through embedding of `PSP1155Data` and implementation of `IAccessControl` and
/// `AccessControlStorage` traits.
#[brush::trait_definition]
pub trait IAccessControl: AccessControlStorage {
    /// Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        self._has_role(&role, &address)
    }

    /// Returns the admin role that controls `role`. See `grant_role` and `revoke_role`.
    #[ink(message)]
    fn get_role_admin(&self, role: RoleType) -> RoleType {
        self._get_role_admin(&role)
    }

    /// Grants `role` to `account`.
    ///
    /// On success a `RoleGranted` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `MissingRole` error if caller can't grant the role.
    #[ink(message)]
    fn grant_role(&mut self, role: RoleType, account: AccountId) {
        self._check_role(&self._get_role_admin(&role), &Self::env().caller());

        if !self._has_role(&role, &account) {
            self.get_mut()
                .roles
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(account, true);
            self._emit_role_granted(role, account, Some(Self::env().caller()))
        }
    }

    /// Revokes `role` from `account`.
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `MissingRole` error if caller can't grant the role.
    #[ink(message)]
    fn revoke_role(&mut self, role: RoleType, account: AccountId) {
        let caller = Self::env().caller();
        self._check_role(&self._get_role_admin(&role), &caller);
        self._do_revoke_role(role, account);
    }

    /// Revokes `role` from the calling account.
    /// Roles are often managed via `grant_role` and `revoke_role`: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `InvalidCaller` error if caller is not `address`.
    #[ink(message)]
    fn renounce_role(&mut self, role: RoleType, address: AccountId) {
        assert_eq!(
            Self::env().caller(),
            address,
            "{}",
            AccessControlError::InvalidCaller.as_ref()
        );
        self._do_revoke_role(role, address);
    }

    // Helper functions

    /// The user must override this function using their event definition.
    fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous_admin_role: RoleType, _new_admin_role: RoleType) {
    }

    /// The user must override this function using their event definition.
    fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>) {}

    /// The user must override this function using their event definition.
    fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId) {}

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
            self.get_mut()
                .roles
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(address, false);
            self._emit_role_revoked(role, address, Self::env().caller());
        }
    }

    fn _check_role(&self, role: &RoleType, address: &AccountId) {
        assert!(
            self._has_role(role, address),
            "{}",
            AccessControlError::MissingRole.as_ref()
        )
    }

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let old_admin = self
            .get_mut()
            .roles
            .entry(role)
            .or_insert_with(RoleData::default)
            .admin_role;
        self._emit_role_admin_changed(role, old_admin, new_admin);
        self.get_mut()
            .roles
            .entry(role)
            .or_insert_with(RoleData::default)
            .admin_role = new_admin;
    }
}
