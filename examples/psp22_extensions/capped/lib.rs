#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_capped {
    use ink_prelude::string::String;
    use psp22::{
        extensions::capped::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22CappedStorage)]
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
            let mut instance = Self::default();
            assert!(instance.init_cap(cap).is_ok());
            assert!(instance._mint(instance.env().caller(), inital_supply).is_ok());
            instance
        }

        /// Expose the `_mint` function
        #[ink(message)]
        pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint(account, amount)
        }

        /// Overrides the `_mint` function to check for cap overflow before minting tokens
        /// Performs `PSP22::_mint` after the check succeeds
        fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            if (self.total_supply() + amount) > self.cap() {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")))
            }
            PSP22::_mint(self, account, amount)
        }
    }
}
