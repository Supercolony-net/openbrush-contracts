#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_capped {
    use openbrush::contracts::psp22::extensions::capped::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22CappedStorage)]
    pub struct MyPSP22Capped {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22CappedStorageField]
        capped: PSP22CappedData,
    }

    impl PSP22 for MyPSP22Capped {}

    impl PSP22Capped for MyPSP22Capped {}

    impl MyPSP22Capped {
        /// Constructor which mints `initial_supply` of the token to sender
        /// Will set the token's cap to `cap`
        #[ink(constructor)]
        pub fn new(inital_supply: Balance, cap: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._init_cap(cap).is_ok());
                assert!(instance.mint(instance.env().caller(), inital_supply).is_ok());
            })
        }

        /// Initializes the token's cap
        fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
            if cap == 0 {
                return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
            }
            PSP22CappedStorage::get_mut(self).cap = cap;
            Ok(())
        }
    }
}
