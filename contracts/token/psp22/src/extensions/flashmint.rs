use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::{
    string::String,
    vec::Vec,
};

#[brush::wrapper]
pub type PSP22FlashMintWrapper = dyn PSP22FlashMint + PSP22;

#[brush::trait_definition]
pub trait PSP22FlashMint: PSP22 {
    const RETURN_VALUE: [u8; 32] = ink_lang::blake2x256!("PSP3156FlashBorrower.onFlashLoan");

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
    /// Returns `Wrong token` error if the `token` account id is not this token
    #[ink(message)]
    fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, PSP22Error> {
        if token != Self::env().account_id() {
            return Err(PSP22Error::Custom(String::from("Wrong token")))
        }
        Ok(self.get_fee(amount))
    }

    /// Mints `amount` of `token` to `receiver_account` and performs the flashloan
    /// `amount` is then burned along with the fee for the flashloan
    ///
    /// `receiver_account` must implement `PSP3156FlashBorrower`
    ///
    /// Returns `Invalid return value` error if the `receiver_account` returns incorrect bytes
    /// Returns `Allowance does not allow refund` error if the contract does not have
    /// enough allowance to transfer borrowed amount and fees from `receiver_account`
    #[ink(message)]
    fn flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let fee = self.flash_fee(token, amount)?;
        self._mint(receiver_account, amount)?;
        self.on_flashloan(receiver_account, token, fee, amount, data)?;
        let current_allowance = self.allowance(receiver_account, Self::env().account_id());
        if current_allowance < amount + fee {
            return Err(PSP22Error::Custom(String::from("Allowance does not allow refund")))
        }
        self._approve_from_to(
            receiver_account,
            Self::env().account_id(),
            current_allowance - amount - fee,
        )?;
        self._burn(receiver_account, amount + fee)
    }

    /// Helper function to get fee for borrowing `amount` of token
    fn get_fee(&mut self, _amount: Balance) -> Balance {
        0
    }

    /// Helper function which calls `on_flashloan` on `receiver_account`
    fn on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        if PSP3156FlashBorrowerWrapper::on_flashloan(&receiver_account, Self::env().caller(), token, amount, fee, data)
            != Self::RETURN_VALUE
        {
            return Err(PSP22Error::Custom(String::from("Invalid return value")))
        }
        Ok(())
    }
}

#[brush::wrapper]
pub type PSP3156FlashBorrowerWrapper = dyn PSP3156FlashBorrower;

#[brush::trait_definition]
pub trait PSP3156FlashBorrower {
    #[ink(message)]
    fn on_flashloan(
        &mut self,
        initiator: AccountId,
        token: AccountId,
        amount: Balance,
        fee: Balance,
        data: Vec<u8>,
    ) -> [u8; 32];
}
