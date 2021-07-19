#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_access_control {
    use psp721::traits::*;
    use access_control::traits::*;
    use brush::modifiers;

    #[ink(storage)]
    #[derive(Default)]
    #[derive(PSP721Storage, AccessControlStorage)]
    pub struct PSP721Struct {}

    // ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
    const MINTER: RoleType = 0xfd9ab216;

    #[brush::modifier_definition]
    pub fn only_minter(instance: &mut PSP721Struct, body: impl Fn(&mut PSP721Struct)) {
        instance._check_role(&MINTER, &instance.env().caller());
        body(instance)
    }

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            instance.grant_role(MINTER, caller);
            instance
        }
    }

    impl IPSP721 for PSP721Struct {}
    impl IAccessControl for PSP721Struct {}

    impl IPSP721Mint for PSP721Struct {
        #[modifiers(only_minter)]
        fn mint(&mut self, id: Id) {
            // We added modifier to function.
            // #[super]self.mint(id) will call default implementation from trait
            #[super]self.mint(id);
        }

        #[modifiers(only_minter)]
        fn burn(&mut self, id: Id) {
            // We added modifier to function.
            // #[super]self.burn(id) will call default implementation from trait
            #[super]self.burn(id);
        }
    }
}
