use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};
pub use common::errors::{
    FlashBorrowerError,
    FlashLenderError,
    PSP22FlashmintError,
};
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};

#[brush::wrapper]
pub type PSP22FlashMintCaller = dyn PSP22FlashMint + PSP22;

#[brush::trait_definition]
pub trait PSP22FlashMint: PSP22 {
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
    fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, PSP22FlashmintError> {
        if token != Self::env().account_id() {
            return Err(PSP22FlashmintError::WrongTokenAddress)
        }
        Ok(self.get_fee(amount))
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
    ) -> Result<(), PSP22FlashmintError> {
        let fee = self.flash_fee(token, amount)?;
        self._mint(receiver_account, amount)?;
        self._on_flashloan(receiver_account, token, fee, amount, data)?;
        let current_allowance = self.allowance(receiver_account, Self::env().account_id());
        if current_allowance < amount + fee {
            return Err(PSP22FlashmintError::AllowanceDoesNotAllowRefund)
        }
        self._approve_from_to(
            receiver_account,
            Self::env().account_id(),
            current_allowance - amount - fee,
        )?;
        self._burn(receiver_account, amount + fee)?;
        Ok(())
    }

    /// Helper function to get fee for borrowing `amount` of token
    fn get_fee(&mut self, _amount: Balance) -> Balance {
        0
    }

    /// Helper function which calls `on_flashloan` on `receiver_account`
    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22FlashmintError> {
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
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => {
                match e {
                    EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
                    _ => {
                        return Err(PSP22FlashmintError::FlashloanRejected(String::from(
                            "Error while performing the flashloan",
                        )))
                    }
                }
            }
        }
    }
}

#[brush::wrapper]
pub type FlashBorrowerCaller = dyn FlashBorrower;

/// Flash Borrower implementation as proposed in https://eips.ethereum.org/EIPS/eip-3156)
#[brush::trait_definition]
pub trait FlashBorrower {
    #[ink(message)]
    fn on_flashloan(
        &mut self,
        initiator: AccountId,
        token: AccountId,
        amount: Balance,
        fee: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashBorrowerError>;
}

#[brush::wrapper]
pub type FlashLenderCaller = dyn FlashLender;

/// Flash Lender implementation as proposed in https://eips.ethereum.org/EIPS/eip-3156)
#[brush::trait_definition]
pub trait FlashLender {
    #[ink(message)]
    fn max_flashloan(&mut self, _token: AccountId) -> Balance;

    #[ink(message)]
    fn flash_fee(&mut self, _token: AccountId, _amount: Balance) -> Result<Balance, FlashLenderError>;

    #[ink(message)]
    fn flashloan(
        &mut self,
        _receiver_account: AccountId,
        _token: AccountId,
        _amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), FlashLenderError>;
}
