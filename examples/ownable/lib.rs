#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod ownable {
    use psp1155::traits::*;
    use ownable::traits::*;
    use brush::{
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, PSP1155Storage, OwnableStorage)]
    pub struct PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }
    }

    impl IOwnable for PSP1155Struct {}
    impl IPSP1155 for PSP1155Struct {}

    impl IPSP1155Mint for PSP1155Struct {
        #[modifiers(only_owner)]
        fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
            // We added modifier to function.
            // #[super]self.mint(to, id, amount) will call default implementation from trait
            #[super]self.mint(to, id, amount);
        }

        #[modifiers(only_owner)]
        fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
            // We added modifier to function.
            // #[super]self.burn(from, id, amount) will call default implementation from trait
            #[super]self.burn(from, id, amount);
        }
    }
}