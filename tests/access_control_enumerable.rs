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
    use ink_storage::traits::SpreadAllocate;

    use openbrush::{
        contracts::access_control::extensions::enumerable::*,
        test_utils::accounts,
    };

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink_lang::selector_id!("MINTER");
    const PAUSER: RoleType = ink_lang::selector_id!("PAUSER");

    #[derive(Default, SpreadAllocate, AccessControlStorage)]
    #[ink(storage)]
    pub struct AccessControlStruct {
        #[AccessControlStorageField]
        access: AccessControlData<EnumerableMembers>,
    }

    impl AccessControl for AccessControlStruct {}

    impl AccessControlEnumerable for AccessControlStruct {}

    impl AccessControlStruct {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                _instance._init_with_admin(admin);
            })
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();

        accounts
    }

    #[ink::test]
    fn should_change_role_member_count() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert_eq!(access_control.get_role_member_count(PAUSER), 0);

        assert!(access_control.grant_role(PAUSER, alice).is_ok());
        assert_eq!(access_control.get_role_member_count(PAUSER), 1);

        assert!(access_control.grant_role(PAUSER, accounts.bob).is_ok());
        assert_eq!(access_control.get_role_member_count(PAUSER), 2);

        assert!(access_control.revoke_role(PAUSER, alice).is_ok());
        assert!(access_control.grant_role(MINTER, alice).is_ok());
        assert_eq!(access_control.get_role_member_count(PAUSER), 1);
        assert_eq!(access_control.get_role_member_count(MINTER), 1);
    }

    #[ink::test]
    fn should_return_role_member() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert!(access_control.grant_role(PAUSER, accounts.bob).is_ok());
        assert!(access_control.grant_role(PAUSER, alice).is_ok());
        assert!(access_control.grant_role(PAUSER, accounts.eve).is_ok());

        assert_eq!(access_control.get_role_member(PAUSER, 1), Some(alice))
    }

    #[ink::test]
    fn get_role_member_fails() {
        let accounts = setup();
        let alice = accounts.alice;
        let mut access_control = AccessControlStruct::new(alice);

        assert!(access_control.grant_role(PAUSER, accounts.bob).is_ok());
        assert!(access_control.grant_role(PAUSER, alice).is_ok());
        assert_eq!(access_control.get_role_member(PAUSER, 1), Some(alice));

        assert!(access_control.revoke_role(PAUSER, alice).is_ok());
        assert_eq!(access_control.get_role_member(PAUSER, 1), None)
    }
}
