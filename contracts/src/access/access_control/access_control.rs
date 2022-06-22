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

pub use super::members::*;
pub use crate::traits::access_control::*;
pub use derive::AccessControlStorage;
use openbrush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    storage::{
        Mapping,
        ValueGuard,
    },
    traits::AccountId,
};

pub const DATA_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::AccessControlData");

#[derive(Default, Debug)]
#[openbrush::storage(DATA_KEY)]
pub struct AccessControlData<B = Members>
where
    B: AccessControlMemberManager,
{
    pub admin_roles: Mapping<RoleType, RoleType, ValueGuard<RoleType>>,
    pub members: B,
    pub _reserved: Option<()>,
}

declare_storage_trait!(AccessControlStorage);

pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

/// Modifier that checks that `caller` has a specific role.
#[modifier_definition]
pub fn only_role<T, B, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    B: AccessControlMemberManager,
    T: AccessControlStorage<Data = AccessControlData<B>>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if let Err(err) = check_role(instance, role, T::env().caller()) {
        return Err(From::from(err))
    }
    body(instance)
}

impl<B, T> AccessControl for T
where
    B: AccessControlMemberManager,
    T: AccessControlStorage<Data = AccessControlData<B>>,
{
    default fn has_role(&self, role: RoleType, address: AccountId) -> bool {
        self.get().members.has_role(role, &address)
    }

    default fn get_role_admin(&self, role: RoleType) -> RoleType {
        get_role_admin(self, role)
    }

    #[modifiers(only_role(get_role_admin(self, role)))]
    default fn grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if self.get().members.has_role(role, &account) {
            return Err(AccessControlError::RoleRedundant)
        }
        self.get_mut().members.add(role, &account);
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

    fn _setup_role(&mut self, role: RoleType, member: AccountId);

    fn _do_revoke_role(&mut self, role: RoleType, account: AccountId);

    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType);
}

impl<B, T> AccessControlInternal for T
where
    B: AccessControlMemberManager,
    T: AccessControlStorage<Data = AccessControlData<B>>,
{
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

    default fn _setup_role(&mut self, role: RoleType, member: AccountId) {
        if !self.get().members.has_role(role, &member) {
            self.get_mut().members.add(role, &member);

            self._emit_role_granted(role, member, None);
        }
    }

    default fn _do_revoke_role(&mut self, role: RoleType, account: AccountId) {
        self.get_mut().members.remove(role, &account);
        self._emit_role_revoked(role, account, Self::env().caller());
    }

    default fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        let mut entry = self.get_mut().admin_roles.get(role);
        if entry.is_none() {
            entry = Some(Self::_default_admin());
        }
        let old_admin = entry.unwrap();
        self.get_mut().admin_roles.insert(role, &new_admin);
        self._emit_role_admin_changed(role, old_admin, new_admin);
    }
}

pub fn check_role<T, B>(instance: &T, role: RoleType, account: AccountId) -> Result<(), AccessControlError>
where
    B: AccessControlMemberManager,
    T: AccessControlStorage<Data = AccessControlData<B>>,
{
    if !instance.get().members.has_role(role, &account) {
        return Err(AccessControlError::MissingRole)
    }
    Ok(())
}

pub fn get_role_admin<T, B>(instance: &T, role: RoleType) -> RoleType
where
    B: AccessControlMemberManager,
    T: AccessControlStorage<Data = AccessControlData<B>>,
{
    instance.get().admin_roles.get(role).unwrap_or(T::_default_admin())
}
