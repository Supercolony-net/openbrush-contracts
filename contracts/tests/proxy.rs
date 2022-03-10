#![feature(min_specialization)]
#[cfg(feature = "proxy")]
#[brush::contract]
mod proxy {
    use brush::{
        test_utils::change_caller
    };
    use contracts::{
        ownable::*, 
        proxy::*
    };
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;


    #[ink(storage)]
    #[derive(Default, OwnableStorage, ProxyStorage)]
    pub struct MyProxy {
        #[OwnableStorageField]
        ownable: OwnableData,
        #[ProxyStorageField]
        proxy: ProxyData,
    }

    impl MyProxy {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            let mut inst = Self::default();
            inst._init_with_forward_to(forward_to);
            inst._init_with_owner(Self::env().caller());
            inst
        }
    }

    impl Proxy for MyProxy {}

    #[ink::test]
    fn implementation_works() {
    }

    #[ink::test]
    fn change_implementation() {
    }

    #[ink::test]
    fn forward() {
    }
}
