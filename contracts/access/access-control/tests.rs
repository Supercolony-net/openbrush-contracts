#[cfg(test)]
#[brush::contract]
mod tests {
    use crate::traits::*;
    use ink_env::test::DefaultAccounts;
    use ::ink_env::{DefaultEnvironment};
    use ink_lang as ink;
    // TODO: Emit events
    // use ink::{Env, EmitEvent};

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;
    // ::ink_lang_ir::Selector::new("PAUSER".as_ref()).as_bytes()
    const PAUSER: RoleType = 0x4ce9afe6;

    #[derive(Default, AccessControlStorage)]
    #[ink(storage)]
    pub struct AccessControlStruct {}

    impl IAccessControl for AccessControlStruct {}

    impl AccessControlStruct {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut instance = Self::default();
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
    fn should_grant_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(PAUSER, accounts.alice);
        access_control.grant_role(MINTER, accounts.alice);

        assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, accounts.alice));
        assert!(access_control.has_role(PAUSER, accounts.alice));
        assert!(access_control.has_role(MINTER, accounts.alice));
    }

    #[ink::test]
    fn should_revoke_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(PAUSER, accounts.bob);
        assert!(access_control.has_role(PAUSER, accounts.bob));
        access_control.revoke_role(PAUSER, accounts.bob);

        assert!(!access_control.has_role(PAUSER, accounts.bob));
    }

    #[ink::test]
    fn should_renounce_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);
        change_caller(accounts.alice);

        access_control.grant_role(PAUSER, accounts.eve);
        assert!(access_control.has_role(PAUSER, accounts.eve));
        change_caller(accounts.eve);
        access_control.renounce_role(PAUSER, accounts.eve);

        assert!(!access_control.has_role(PAUSER, accounts.eve));
    }

    #[ink::test]
    fn should_change_role_admin() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(MINTER, accounts.eve);
        access_control._set_role_admin(PAUSER, MINTER);
        change_caller(accounts.eve);
        access_control.grant_role(PAUSER, accounts.bob);

        assert_eq!(access_control.get_role_admin(MINTER), DEFAULT_ADMIN_ROLE);
        assert_eq!(access_control.get_role_admin(PAUSER), MINTER);
    }

    #[ink::test]
    #[should_panic(expected = "MissingRole")]
    fn should_return_error_when_not_admin_grant_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(MINTER, accounts.eve);
        access_control.grant_role(PAUSER, accounts.bob);
        access_control._set_role_admin(PAUSER, MINTER);

        access_control.grant_role(PAUSER, accounts.eve);
    }

    #[ink::test]
    #[should_panic(expected = "MissingRole")]
    fn should_return_error_when_not_admin_revoke_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(MINTER, accounts.eve);
        access_control.grant_role(PAUSER, accounts.bob);
        access_control._set_role_admin(PAUSER, MINTER);

        change_caller(accounts.bob);

        access_control.revoke_role(MINTER, accounts.bob);
    }

    #[ink::test]
    #[should_panic(expected = "InvalidCaller")]
    fn should_return_error_when_not_self_renounce_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(PAUSER, accounts.bob);
        access_control.renounce_role(PAUSER, accounts.bob);
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