use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::{PackedLayout, SpreadLayout},
    Box,
};
use utils::{
    traits::{InkStorage, AccountId},
    define_getters,
};
use crate::traits::{ AccessControlError, RoleType };

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

pub trait AccessControlStorage: InkStorage {
    // Mapping of roles to role data which contains information about members of role
    define_getters!(_roles, _roles_mut, StorageHashMap<RoleType, RoleData>);
}

pub trait AccessControl: AccessControlStorage {
    fn _init_with_admin(&mut self, admin: AccountId) {
        self._roles_mut().insert(DEFAULT_ADMIN_ROLE, RoleData::new(admin));
        // TODO: Emit event
    }
    
    fn _init(&mut self) {
        let caller = Self::env().caller();
        self._init_with_admin(caller);
    }

    fn _set_role_admin(&mut self, role: RoleType, admin_role: RoleType) -> Result<(), AccessControlError> {
        self._roles_mut()
            .entry(role)
            .or_insert_with(RoleData::default)
            .admin_role = admin_role;
        // TODO: Emit event
        Ok(())
    }

    fn _grant_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError> {
        self._check_role(&self._get_role_admin(&role), &Self::env().caller())?;

        if !self._has_role(&role, &address) {
            self._roles_mut()
                .entry(role)
                .or_insert_with(RoleData::default)
                .members
                .insert(address, true);
            // TODO: Emit event
        }
        Ok(())
    }

    fn _revoke_role(
        &mut self,
        role: RoleType,
        address: AccountId,
    ) -> Result<(), AccessControlError> {
        let caller = Self::env().caller();
        self._check_role(&self._get_role_admin(&role), &caller)?;

        self._do_revoke_role(role, address);
        Ok(())
    }

    fn _renounce_role(
        &mut self,
        role: RoleType,
        address: AccountId,
    ) -> Result<(), AccessControlError> {
        let caller = Self::env().caller();
        if caller != address {
            return Err(AccessControlError::InvalidCaller);
        }

        self._do_revoke_role(role, address);
        Ok(())
    }

    fn _check_role(&self, role: &RoleType, address: &AccountId) -> Result<(), AccessControlError> {
        if !self._has_role(role, address) {
            return Err(AccessControlError::MissingRole);
        }
        Ok(())
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
}

#[cfg(test)]
#[ink_lang::contract]
mod tests {
    use super::*;
    use ink_env::test::DefaultAccounts;
    use ::ink_env::{DefaultEnvironment};
    use ink_lang as ink;
    use utils::{
        traits::{InkStorage},
        iml_getters, assert_err, assert_ok
    };
    // TODO: Emit events
    // use ink::{Env, EmitEvent};
    use crate::traits::{ IAccessControl };

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;
    // ::ink_lang_ir::Selector::new("PAUSER".as_ref()).as_bytes()
    const PAUSER: RoleType = 0x4ce9afe6;

    #[derive(Default)]
    #[ink(storage)]
    pub struct AccessControlStruct {
        roles: StorageHashMap<RoleType, RoleData>,
    }

    impl InkStorage for AccessControlStruct {
        fn _empty() -> Self {
            let mut instance = Self::default();
            instance._init();
            instance
        }
    }
    impl AccessControlStorage for AccessControlStruct {
        iml_getters!(roles, _roles, _roles_mut, StorageHashMap<RoleType, RoleData>);
    }
    impl AccessControl for AccessControlStruct {}
    
    impl IAccessControl for AccessControlStruct {
        #[ink(message)]
        fn has_role(&self, role: RoleType, address: AccountId) -> bool {
            self._has_role(&role, &address)
        }

        #[ink(message)]
        fn get_role_admin(&self, role: RoleType) -> RoleType {
            self._get_role_admin(&role)
        }

        #[ink(message)]
        fn grant_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError> {
            self._grant_role(role, address)
        }

        #[ink(message)]
        fn revoke_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError> {
            self._revoke_role(role, address)
        }

        #[ink(message)]
        fn renounce_role(&mut self, role: RoleType, address: AccountId) -> Result<(), AccessControlError> {
            self._renounce_role(role, address)
        }
    }

    impl AccessControlStruct {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut instance = Self::_empty();
            instance._init_with_admin(admin);
            instance
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        accounts
    }

    #[ink::test]
    fn should_init_with_default_admin() {
        let accounts = setup();
        let access_control = AccessControlStruct::new(accounts.alice);
        assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, accounts.alice));
        assert_eq!(
            access_control.get_role_admin(DEFAULT_ADMIN_ROLE),
            DEFAULT_ADMIN_ROLE
        );
    }

    #[ink::test]
    fn should_init_with_defined_role() {
        let accounts = setup();
        let access_control = AccessControlStruct::new(accounts.alice);

        assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, accounts.alice));
        assert_eq!(
            access_control.get_role_admin(DEFAULT_ADMIN_ROLE),
            DEFAULT_ADMIN_ROLE
        );
    }

    #[ink::test]
    fn should_grant_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        assert_ok!(access_control.grant_role(PAUSER, accounts.alice));
        assert_ok!(access_control.grant_role(MINTER, accounts.alice));

        assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, accounts.alice));
        assert!(access_control.has_role(PAUSER, accounts.alice));
        assert!(access_control.has_role(MINTER, accounts.alice));
    }

    #[ink::test]
    fn should_revoke_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        assert_ok!(access_control.grant_role(PAUSER, accounts.bob));
        assert!(access_control.has_role(PAUSER, accounts.bob));
        assert_ok!(access_control.revoke_role(PAUSER, accounts.bob));

        assert!(!access_control.has_role(PAUSER, accounts.bob));
    }

    #[ink::test]
    fn should_renounce_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);
        change_caller(accounts.alice);

        assert_ok!(access_control.grant_role(PAUSER, accounts.eve));
        assert!(access_control.has_role(PAUSER, accounts.eve));
        change_caller(accounts.eve);
        assert_ok!(access_control.renounce_role(PAUSER, accounts.eve));

        assert!(!access_control.has_role(PAUSER, accounts.eve));
    }

    #[ink::test]
    fn should_change_role_admin() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        assert_ok!(access_control.grant_role(MINTER, accounts.eve));
        assert_ok!(access_control._set_role_admin(PAUSER, MINTER));
        change_caller(accounts.eve);
        assert_ok!(access_control.grant_role(PAUSER, accounts.bob));

        assert_eq!(access_control.get_role_admin(MINTER), DEFAULT_ADMIN_ROLE);
        assert_eq!(access_control.get_role_admin(PAUSER), MINTER);
    }

    #[ink::test]
    fn should_return_error_when_not_admin_grant_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        assert_ok!(access_control.grant_role(MINTER, accounts.eve));
        assert_ok!(access_control.grant_role(PAUSER, accounts.bob));
        assert_ok!(access_control._set_role_admin(PAUSER, MINTER));

        assert_err!(
                access_control.grant_role(PAUSER, accounts.eve),
                AccessControlError::MissingRole
            );
    }

    #[ink::test]
    fn should_return_error_when_not_admin_revoke_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        assert_ok!(access_control.grant_role(MINTER, accounts.eve));
        assert_ok!(access_control.grant_role(PAUSER, accounts.bob));
        assert_ok!(access_control._set_role_admin(PAUSER, MINTER));

        change_caller(accounts.bob);

        assert_err!(
                access_control.revoke_role(MINTER, accounts.bob),
                AccessControlError::MissingRole
            );
    }

    #[ink::test]
    fn should_return_error_when_not_self_renounce_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        assert_ok!(access_control.grant_role(PAUSER, accounts.bob));
        assert_err!(
                access_control.renounce_role(PAUSER, accounts.bob),
                AccessControlError::InvalidCaller
            );
    }

    fn change_caller(new_caller: AccountId) {
        // CHANGE CALLEE MANUALLY
        // Get contract address.
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
        // Create call.
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&new_caller);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            new_caller,
            callee,
            1000000,
            1000000,
            data,
        );
    }
}