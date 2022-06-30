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
                assert!(instance.init_cap(cap).is_ok());
                assert!(instance._mint_to(instance.env().caller(), inital_supply).is_ok());
            })
        }

        /// Overrides the `_mint_to` function to check for cap overflow before minting tokens
        /// Performs `PSP22::_mint_to` after the check succeeds
        fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            if (self.total_supply() + amount) > self.cap() {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")))
            }
            PSP22Internal::_mint_to(self, account, amount)
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
