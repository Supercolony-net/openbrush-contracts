// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    access_control,
    access_control::members,
    traits::access_control::*,
};
pub use access_control::Internal as _;
use ink::storage::traits::Storable;

use openbrush::{
    modifier_definition,
    modifiers,
    storage::{
        Mapping,
        ValueGuard,
    },
    traits::{
        AccountId,
        OccupiedStorage,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data<M = members::Members>
where
    M: members::MembersManager + Storable,
{
    pub admin_roles: Mapping<RoleType, RoleType, ValueGuard<RoleType>>,
    pub members: M,
    pub _reserved: Option<()>,
}

pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

/// Modifier that checks that `caller` has a specific role.
#[modifier_definition]
pub fn only_role<T, M, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    M: members::MembersManager + Storable,
    T: Storage<Data<M>>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if let Err(err) = check_role(instance, role, T::env().caller()) {
        return Err(From::from(err))
    }
    body(instance)
}

impl<T, M> AccessControl for T
where
    M: members::MembersManager + Storable,
    T: Storage<Data<M>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<M>>,
{
    default fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        self.data().members.has_role(role, &address)
    }

    default fn get_role_admin(&self, role: RoleType) -> RoleType {
        get_role_admin(self, role)
    }

    #[modifiers(only_role(get_role_admin(self, role)))]
    default fn grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if self.data().members.has_role(role, &account) {
            return Err(AccessControlError::RoleRedundant)
        }
        self.data().members.add(role, &account);
        self._emit_role_granted(role, account, Some(T::env().caller()));
        Ok(())
    }

    #[modifiers(only_role(get_role_admin(self, role)))]
    default fn revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        check_role(self, role, account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }

    default fn renounce_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if Self::env().caller() != account {
            return Err(AccessControlError::InvalidCaller)
        }
        check_role(self, role, account)?;
        self._do_revoke_role(role, account);
        Ok(())
    }
}

pub trait Internal {
    /// The user must override those methods using their event definition.
    fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous: RoleType, _new: RoleType);
    fn _emit_role_granted(&mut self, _role: RoleType, _grantee: AccountId, _grantor: Option<AccountId>);
    fn _emit_role_revoked(&mut self, _role: RoleType, _account: AccountId, _sender: AccountId);

    fn _default_admin() -> RoleType;

    fn _init_with_caller(&mut self);

    fn _init_with_admin(&mut self, admin: AccountId);

    fn _setup_role(&mut self, role: RoleType, member: AccountId);

    fn _do_revoke_role(&mut self, role: RoleType, account: AccountId);

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType);
}

impl<T, M> Internal for T
where
    M: members::MembersManager + Storable,
    T: Storage<Data<M>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<M>>,
{
    default fn _emit_role_admin_changed(&mut self, _role: RoleType, _previous: RoleType, _new: RoleType) {}
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

    default fn _setup_role(&mut self, role: RoleType, member: AccountId) {
        if !self.data().members.has_role(role, &member) {
            self.data().members.add(role, &member);

            self._emit_role_granted(role, member, None);
        }
    }

    default fn _do_revoke_role(&mut self, role: RoleType, account: AccountId) {
        self.data().members.remove(role, &account);
        self._emit_role_revoked(role, account, Self::env().caller());
    }

    default fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let mut entry = self.data().admin_roles.get(role);
        if entry.is_none() {
            entry = Some(Self::_default_admin());
        }
        let old_admin = entry.unwrap();
        self.data().admin_roles.insert(role, &new_admin);
        self._emit_role_admin_changed(role, old_admin, new_admin);
    }
}

pub fn check_role<T, M>(instance: &T, role: RoleType, account: AccountId) -> Result<(), AccessControlError>
where
    M: members::MembersManager + Storable,
    T: Storage<Data<M>>,
{
    if !instance.data().members.has_role(role, &account) {
        return Err(AccessControlError::MissingRole)
    }
    Ok(())
}

pub fn get_role_admin<T, M>(instance: &T, role: RoleType) -> RoleType
where
    M: members::MembersManager + Storable,
    T: Storage<Data<M>> + Internal,
{
    instance.data().admin_roles.get(role).unwrap_or(T::_default_admin())
}
