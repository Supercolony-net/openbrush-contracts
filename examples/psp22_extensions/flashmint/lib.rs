#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_flashmint {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::flashmint::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct MyPSP22FlashMint {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPSP22FlashMint {}

    impl FlashLender for MyPSP22FlashMint {
        #[ink(message)]
        fn max_flashloan(&mut self, token: AccountId) -> Balance {
            self._max_flashloan(token)
        }

        #[ink(message)]
        fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
            match self._flash_fee(token, amount) {
                Ok(result) => Ok(result),
                Err(e) => Err(e.into()),
            }
        }

        #[ink(message)]
        fn flashloan(
            &mut self,
            receiver_account: AccountId,
            token: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), FlashLenderError> {
            self._flashloan(receiver_account, token, amount, data)?;
            Ok(())
        }
    }

    impl PSP22FlashMint for MyPSP22FlashMint {
        /// Override `get_fee` function to add 1% fee to the borrowed `amount`
        fn _get_fee(&mut self, amount: Balance) -> Balance {
            amount / 100
        }
    }

    impl MyPSP22FlashMint {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }
    }
}
