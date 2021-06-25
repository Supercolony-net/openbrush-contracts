#[cfg(test)]
#[brush::contract]
mod tests {
    use crate::traits::{RoleType, IAccessControl};
    use crate::impls::{AccessControlStorage, AccessControl, RoleData, DEFAULT_ADMIN_ROLE, StorageHashMap};
    use ink_env::test::DefaultAccounts;
    use ::ink_env::{DefaultEnvironment};
    use ink_lang as ink;
    use brush::{
        traits::{InkStorage},
    };

    use ink::{Env, EmitEvent};

    #[ink(event)]
    pub struct RoleAdminChanged {
        #[ink(topic)]
        role: RoleType,
        #[ink(topic)]
        previous_admin_role: RoleType,
        #[ink(topic)]
        new_admin_role: RoleType
    }

    #[ink(event)]
    pub struct RoleGranted {
        #[ink(topic)]
        role: RoleType,
        #[ink(topic)]
        grantee: AccountId,
        #[ink(topic)]
        grantor: Option<AccountId>
    }

    #[ink(event)]
    pub struct RoleRevoked {
        #[ink(topic)]
        role: RoleType,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        admin: AccountId
    }

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;
    // ::ink_lang_ir::Selector::new("PAUSER".as_ref()).as_bytes()
    const PAUSER: RoleType = 0x4ce9afe6;

    #[derive(Default, AccessControlStorage, IAccessControl)]
    #[ink(storage)]
    pub struct AccessControlStruct {}

    type Event = <AccessControlStruct as ::ink_lang::BaseEvent>::Type;

    impl AccessControl for AccessControlStruct {
        fn emit_role_admin_changed(&mut self, role: u32, previous_admin_role: u32, new_admin_role: u32) {
            Self::env().emit_event(RoleAdminChanged {
                role,
                previous_admin_role,
                new_admin_role
            })
        }

        fn emit_role_granted(&mut self, role: u32, grantee: AccountId, grantor: Option<AccountId>) {
            Self::env().emit_event(RoleGranted {
                role,
                grantee,
                grantor
            })
        }

        fn emit_role_revoked(&mut self, role: u32, account: AccountId, sender: AccountId) {
            Self::env().emit_event(RoleRevoked {
                role,
                account,
                admin: sender
            })
        }
    }

    impl AccessControlStruct {
        pub fn new(admin: AccountId) -> impl AccessControl {
            Self::constructor(admin)
        }

        #[ink(constructor)]
        pub fn constructor(admin: AccountId) -> Self {
            let mut instance = Self::default();
            instance._init_with_admin(admin);
            instance
        }
    }

    fn assert_role_admin_change_event(event: &ink_env::test::EmittedEvent,
    expected_role: RoleType, expected_prev_admin: RoleType, expected_new_admin: RoleType) {
        if let Event::RoleAdminChanged(RoleAdminChanged {role, previous_admin_role, new_admin_role}) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer") {
            assert_eq!(role, expected_role, "Roles were not equal: encountered role {:?}, expected role {:?}", role, expected_role);
            assert_eq!(previous_admin_role, expected_prev_admin,
                       "Previous admins were not equal: encountered previous admin {:?}, expected {:?}", previous_admin_role, expected_prev_admin);
            assert_eq!(new_admin_role, expected_new_admin,
                       "New admins were not equal: encountered new admin {:?}, expected {:?}", new_admin_role, expected_new_admin);
        }
    }

    fn assert_role_granted_event(event: &ink_env::test::EmittedEvent,
    expected_role: RoleType, expected_grantee: AccountId, expected_grantor: Option<AccountId>) {
        if let Event::RoleGranted(RoleGranted {role, grantee, grantor}) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer") {
            assert_eq!(role, expected_role, "Roles were not equal: encountered role {:?}, expected role {:?}", role, expected_role);
            assert_eq!(grantee, expected_grantee,
                       "Grantees were not equal: encountered grantee {:?}, expected {:?}", grantee, expected_grantee);
            assert_eq!(grantor, expected_grantor,
                       "Grantors were not equal: encountered grantor {:?}, expected {:?}", grantor, expected_grantor);
        }
    }

    fn assert_role_revoked_event(event: &ink_env::test::EmittedEvent,
                                 expected_role: RoleType, expected_account: AccountId, expected_admin: AccountId) {
        if let Event::RoleRevoked(RoleRevoked {role, account, admin}) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer") {
            assert_eq!(role, expected_role, "Roles were not equal: encountered role {:?}, expected role {:?}", role, expected_role);
            assert_eq!(account, expected_account,
                       "Accounts were not equal: encountered account {:?}, expected {:?}", account, expected_account);
            assert_eq!(admin, expected_admin,
                       "Admins were not equal: encountered admin {:?}, expected {:?}", admin, expected_admin);
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
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_role_granted_event(&emitted_events[0], DEFAULT_ADMIN_ROLE, accounts.alice, None);
    }

    #[ink::test]
    fn should_grant_role() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        access_control.grant_role(PAUSER, alice);
        access_control.grant_role(MINTER, alice);

        assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, alice));
        assert!(access_control.has_role(PAUSER, alice));
        assert!(access_control.has_role(MINTER, alice));

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_role_granted_event(&emitted_events[0], DEFAULT_ADMIN_ROLE, alice, None);
        assert_role_granted_event(&emitted_events[1], PAUSER, alice, Some(alice));
        assert_role_granted_event(&emitted_events[2], MINTER, alice, Some(alice));
    }

    #[ink::test]
    fn should_revoke_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(PAUSER, accounts.bob);
        assert!(access_control.has_role(PAUSER, accounts.bob));
        access_control.revoke_role(PAUSER, accounts.bob);

        assert!(!access_control.has_role(PAUSER, accounts.bob));

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_role_granted_event(&emitted_events[0], DEFAULT_ADMIN_ROLE, accounts.alice, None);
        assert_role_granted_event(&emitted_events[1], PAUSER, accounts.bob, Some(accounts.alice));
        assert_role_revoked_event(&emitted_events[2], PAUSER, accounts.bob, accounts.alice);

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

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_role_granted_event(&emitted_events[0], DEFAULT_ADMIN_ROLE, accounts.alice, None);
        assert_role_granted_event(&emitted_events[1], PAUSER, accounts.eve, Some(accounts.alice));
        assert_role_revoked_event(&emitted_events[2], PAUSER, accounts.eve, accounts.eve);

    }

    #[ink::test]
    fn should_change_role_admin() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(MINTER, accounts.eve);
        access_control.set_role_admin(PAUSER, MINTER);
        change_caller(accounts.eve);
        access_control.grant_role(PAUSER, accounts.bob);

        assert_eq!(access_control.get_role_admin(MINTER), DEFAULT_ADMIN_ROLE);
        assert_eq!(access_control.get_role_admin(PAUSER), MINTER);

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_role_granted_event(&emitted_events[0], DEFAULT_ADMIN_ROLE, accounts.alice, None);
        assert_role_granted_event(&emitted_events[1], MINTER, accounts.eve, Some(accounts.alice));
        assert_role_admin_change_event(&emitted_events[2], PAUSER, DEFAULT_ADMIN_ROLE, MINTER);
        assert_role_granted_event(&emitted_events[3], PAUSER, accounts.bob, Some(accounts.eve));
    }

    #[ink::test]
    #[should_panic(expected = "MissingRole")]
    fn should_return_error_when_not_admin_grant_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(MINTER, accounts.eve);
        access_control.grant_role(PAUSER, accounts.bob);
        access_control.set_role_admin(PAUSER, MINTER);

        access_control.grant_role(PAUSER, accounts.eve);
    }

    #[ink::test]
    #[should_panic(expected = "MissingRole")]
    fn should_return_error_when_not_admin_revoke_role() {
        let accounts = setup();
        let mut access_control = AccessControlStruct::new(accounts.alice);

        access_control.grant_role(MINTER, accounts.eve);
        access_control.grant_role(PAUSER, accounts.bob);
        access_control.set_role_admin(PAUSER, MINTER);

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