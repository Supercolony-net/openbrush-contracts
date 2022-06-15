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

#![feature(min_specialization)]
#[cfg(feature = "access_control")]
#[openbrush::contract]
mod access_control_enumerable {
    use ::ink_env::DefaultEnvironment;
    use ink_env::test::DefaultAccounts;
    use ink_lang as ink;
    use openbrush::{
        contracts::access_control_enumerable::*,
        test_utils::{
            accounts,
            change_caller,
        },
    };

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink_lang::selector_id!("MINTER");
    const PAUSER: RoleType = ink_lang::selector_id!("PAUSER");

    #[derive(Default, AccessControlStorage, AccessControlEnumerableStorage)]
    #[ink(storage)]
    pub struct AccessControlStruct {
        #[AccessControlStorageField]
        access: AccessControlData,
        #[AccessControlEnumerableStorageField]
        access_enumerable: AccessControlEnumerableData,
    }

    impl AccessControl for AccessControlStruct {}

    impl AccessControlInternal for AccessControlStruct {}

    impl AccessControlEnumerable for AccessControlStruct {}

    impl AccessControlStruct {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut instance = Self::default();
            instance._init_with_admin(admin);
            instance
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();

        accounts
    }

    #[ink::test]
    fn should_grant_role() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert_eq!(access_control.get_role_member_count(PAUSER), 0);
        assert_eq!(access_control.grant_role(PAUSER, alice), Ok(()));
        assert_eq!(access_control.get_role_member_count(PAUSER), 1);
        assert_eq!(access_control.get_role_member(PAUSER, 1), Ok(alice));
        // assert!(access_control.grant_role_enumerable(MINTER, alice).is_ok());
        //
        // assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, alice));
        assert!(access_control.has_role(PAUSER, accounts.alice));
        // assert!(access_control.has_role(MINTER, alice));
    }

    // #[ink::test]
    // fn should_revoke_role() {
    //     let accounts = setup();
    //     let mut access_control = AccessControlStruct::new(accounts.alice);
    //
    //     assert!(access_control.grant_role(PAUSER, accounts.bob).is_ok());
    //     assert!(access_control.has_role(PAUSER, accounts.bob));
    //     assert_eq!(access_control.revoke_role(PAUSER, accounts.bob), Ok(()));
    //
    //     assert!(!access_control.has_role(PAUSER, accounts.bob));
    // }

    // #[ink::test]
    // fn should_renounce_role() {
    //     let accounts = setup();
    //     let mut access_control = AccessControlStruct::new(accounts.alice);
    //     change_caller(accounts.alice);
    //
    //     assert!(access_control.grant_role_enumerable(PAUSER, accounts.eve).is_ok());
    //     assert!(access_control.has_role(PAUSER, accounts.eve));
    //     change_caller(accounts.eve);
    //     assert!(access_control.renounce_role(PAUSER, accounts.eve).is_ok());
    //
    //     assert!(!access_control.has_role(PAUSER, accounts.eve));
    // }
}