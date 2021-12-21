#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_access_control {
    use brush::{
        contracts::access_control::*,
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, AccessControlStorage)]
    pub struct MyAccessControl {
        #[AccessControlStorageField]
        access: AccessControlData,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const CALLER: RoleType = ink_lang::selector_id!("CALLER");

    impl AccessControl for MyAccessControl {}

    impl MyAccessControl {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            // We grant counter role to caller in constructor, so they can increase the count
            instance.grant_role(CALLER, caller).expect("Should grant the role");
            instance
        }

        #[ink(message)]
        #[modifiers(only_role(CALLER))]
        pub fn restricted_function(&mut self) -> Result<(), AccessControlError> {
            todo!()
        }
    }
}
