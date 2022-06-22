#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::access_control::extensions::enumerable::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, AccessControlStorage)]
    pub struct AccessControlStruct {
        #[AccessControlStorageField]
        access: AccessControlData<EnumerableMembers>,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink_lang::selector_id!("MINTER");

    impl AccessControl for AccessControlStruct {}

    impl AccessControlEnumerable for AccessControlStruct {}

    impl AccessControlStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                let caller = _instance.env().caller();
                _instance._init_with_admin(caller);
                // We grant minter role to caller in constructor, so he can mint/burn tokens
                _instance.grant_role(MINTER, caller).expect("Should grant MINTER role");
                assert_eq!(_instance.get_role_member_count(MINTER), 1);
            })
        }
    }
}
