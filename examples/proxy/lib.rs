#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod proxy {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::{
        ownable::*,
        proxy::*,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, ProxyStorage)]
    pub struct ProxyStruct {
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
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            ProxyInternal::_fallback(self);
        }
    }

    impl Ownable for ProxyStruct {}

    impl Proxy for ProxyStruct {}
}
