#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod proxy {
    use brush::{
        contracts::{
            ownable::*, 
            proxy::*
        },
        modifiers,
    };
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, ProxyStorage)]
    pub struct ProxyStruct {
        #[OwnableStorageField]
        ownable: OwnableData,
        #[ProxyStorageField]
        proxy: ProxyData,
    }

    impl ProxyStruct {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_forward_to(forward_to);
                instance._init_with_owner(caller);
            })
        }
    }

    impl Proxy for ProxyStruct {}
}
