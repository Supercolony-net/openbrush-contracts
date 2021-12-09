pub use crate::traits::access_control::*;
use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::AccountId,
};
pub use derive::AccessControlStorage;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct AccessControlData {
    pub admin_roles: StorageHashMap<RoleType, RoleType>,
    pub members: StorageHashMap<(RoleType, AccountId), ()>,
}

declare_storage_trait!(AccessControlStorage, AccessControlData);

pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

/// Modifier that checks that `caller` has a specific role.
#[modifier_definition]
pub fn only_role<T, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    T: AccessControlStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if let Err(err) = check_role(instance, &role, &T::env().caller()) {
        return Err(From::from(err))
    }
    body(instance)
}

impl<T: AccessControlStorage> AccessControl for T {
    default fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        has_role(self, &role, &address)
    }

    default fn get_role_admin(&self, role: RoleType) -> RoleType {
        get_role_admin(self, &role)
    }

    #[modifiers(only_role(get_role_admin(self, &role)))]
    default fn grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if has_role(self, &role, &account) {
            return Err(AccessControlError::RoleRedundant)
        }
        self.get_mut().members.insert((role.clone(), account.clone()), ());
        self._emit_role_granted(role, account, Some(Self::env().caller()));
        Ok(())
    }

    #[modifiers(only_role(get_role_admin(self, &role)))]
    default fn revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        check_role(self, &role, &account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }

    default fn renounce_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if Self::env().caller() != account {
            return Err(AccessControlError::InvalidCaller)
        }
        check_role(self, &role, &account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }
}

pub trait AccessControlInternal {
    /// The user must override this function using their event definition.
    fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous_admin_role: RoleType, _new_admin_role: RoleType);

    /// The user must override this function using their event definition.
    fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>);

    /// The user must override this function using their event definition.
    fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId);

    fn _default_admin() -> RoleType;

    fn _init_with_caller(&mut self);

    fn _init_with_admin(&mut self, admin: AccountId);

    fn _setup_role(&mut self, role: RoleType, admin: AccountId);

    fn _do_revoke_role(&mut self, role: RoleType, account: AccountId);

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType);
}

impl<T: AccessControlStorage> AccessControlInternal for T {
    default fn _emit_role_admin_changed(
        &mut self,
        _role: RoleType,
        _previous_admin_role: RoleType,
        _new_admin_role: RoleType,
    ) {
    }

    default fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>) {}

    default fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId) {}

    default fn _default_admin() -> RoleType {
        DEFAULT_ADMIN_ROLE
    }

    default fn _init_with_caller(&mut self) {
        let caller = Self::env().caller();
        self._init_with_admin(caller);
    }

    default fn _init_with_admin(&mut self, admin: AccountId) {
        self._setup_role(Self::_default_admin(), admin);
    }

    default fn _setup_role(&mut self, role: RoleType, admin: AccountId) {
        if !has_role(self, &role, &admin) {
            self.get_mut().members.insert((role.clone(), admin.clone()), ());

            self._emit_role_granted(role, admin, None);
        }
    }

    default fn _do_revoke_role(&mut self, role: RoleType, account: AccountId) {
        self.get_mut().members.take(&(role, account));
        self._emit_role_revoked(role, account, Self::env().caller());
    }

    default fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let entry = self.get_mut().admin_roles.entry(role).or_insert(Self::_default_admin());
        let old_admin = entry.clone();
        *entry = new_admin;
        self._emit_role_admin_changed(role, old_admin, new_admin);
    }
}

pub fn check_role<T: AccessControlStorage>(
    instance: &T,
    role: &RoleType,
    account: &AccountId,
) -> Result<(), AccessControlError> {
    if !has_role(instance, role, account) {
        return Err(AccessControlError::MissingRole)
    }
    Ok(())
}

pub fn has_role<T: AccessControlStorage>(instance: &T, role: &RoleType, account: &AccountId) -> bool {
    instance.get().members.contains_key(&(role.clone(), account.clone()))
}

pub fn get_role_admin<T: AccessControlStorage>(instance: &T, role: &RoleType) -> RoleType {
    instance
        .get()
        .admin_roles
        .get(role)
        .cloned()
        .unwrap_or(T::_default_admin())
}
