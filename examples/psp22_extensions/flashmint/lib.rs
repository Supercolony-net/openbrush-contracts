#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_flashmint {
    use brush::traits::{
        Flush,
        InkStorage,
    };
    use ink_env::Error as EnvError;
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
        /// Maximum amount of `token` available to mint
        /// Bounded by the max value of Balance (u128)
        #[ink(message)]
        fn max_flashloan(&mut self, token: AccountId) -> Balance {
            if token == Self::env().account_id() {
                Balance::MAX - self.total_supply()
            } else {
                0
            }
        }

        /// Fee for borrowing `amount` of the `token`
        ///
        /// Returns `WrongTokenAddress` error if the `token` account id is not this token
        #[ink(message)]
        fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
            if token != Self::env().account_id() {
                return Err(PSP22FlashmintError::WrongTokenAddress.into())
            }
            Ok(amount / 100)
        }

        /// Mints `amount` of `token` to `receiver_account` and performs the flashloan
        /// `amount` is then burned along with the fee for the flashloan
        ///
        /// `receiver_account` must implement `PSP3156FlashBorrower`
        ///
        /// Returns `AllowanceDoesNotAllowRefund` error if the contract does not have
        /// enough allowance to transfer borrowed amount and fees from `receiver_account`
        #[ink(message)]
        fn flashloan(
            &mut self,
            receiver_account: AccountId,
            token: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), FlashLenderError> {
            let fee = self.flash_fee(token, amount)?;
            self._mint(receiver_account, amount)?;
            self.flush();
            match FlashBorrowerCaller::on_flashloan_builder(
                &receiver_account,
                Self::env().caller(),
                token,
                amount,
                fee,
                data,
            )
            .fire()
            {
                Ok(result) => {
                    match result {
                        Ok(_) => Ok(()),
                        Err(_) => {
                            Err(FlashLenderError::FlashloanRejected(String::from(
                                "Error while performing the flashloan",
                            )))
                        }
                    }
                }
                Err(e) => {
                    match e {
                        EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
                        _ => {
                            Err(FlashLenderError::FlashloanRejected(String::from(
                                "Error while performing the flashloan",
                            )))
                        }
                    }
                }
            }?;
            self.load();
            let current_allowance = self.allowance(receiver_account, Self::env().account_id());
            if current_allowance < amount + fee {
                return Err(PSP22FlashmintError::AllowanceDoesNotAllowRefund.into())
            }
            self._approve_from_to(
                receiver_account,
                Self::env().account_id(),
                current_allowance - amount - fee,
            )?;
            self._burn(receiver_account, amount + fee)?;
            Ok(())
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
