#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_capped {
    use openbrush::contracts::psp22::extensions::capped::*;
    use openbrush::contracts::psp22::extensions::mintable::*;
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

    impl PSP22Mintable for MyPSP22Capped {}

    impl PSP22Transfer for MyPSP22Capped {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            self._before_mint(*amount)?;
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            Ok(())
        }
    }

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
    }
}
