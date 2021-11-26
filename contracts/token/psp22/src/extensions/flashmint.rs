use crate::traits::*;
use brush::traits::{
    AccountId,
    Balance,
};
pub use common::errors::{
    FlashBorrowerError,
    FlashLenderError,
};
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};

#[brush::wrapper]
pub type PSP22FlashMintCaller = dyn FlashLender + PSP22;

#[brush::trait_definition]
pub trait PSP22FlashMint: PSP22 + FlashLender {
    /// Call this function in `max_flashloan` function in `impl` block of FlashLender
    /// Maximum amount of `token` available to mint
    /// Bounded by the max value of Balance (u128)
    fn _max_flashloan(&mut self, token: AccountId) -> Balance {
        if token == Self::env().account_id() {
            Balance::MAX - self.total_supply()
        } else {
            0
        }
    }

    /// Call this function in `flash_fee` function in `impl` block of FlashLender
    /// Fee for borrowing `amount` of the `token`
    ///
    /// Returns `WrongTokenAddress` error if the `token` account id is not this token
    fn _flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
        if token != Self::env().account_id() {
            return Err(FlashLenderError::WrongTokenAddress)
        }
        Ok(self.get_fee(amount))
    }

    /// Call this function in `flashloan` function in `impl` block of FlashLender
    /// Mints `amount` of `token` to `receiver_account` and performs the flashloan
    /// `amount` is then burned along with the fee for the flashloan
    /// `receiver_account` must implement `PSP3156FlashBorrower`
    ///
    /// Returns `AllowanceDoesNotAllowRefund` error if the contract does not have
    /// enough allowance to transfer borrowed amount and fees from `receiver_account`
    fn _flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let fee = self._flash_fee(token, amount)?;
        self._mint(receiver_account, amount)?;
        self._on_flashloan(receiver_account, token, fee, amount, data)?;
        let current_allowance = self.allowance(receiver_account, Self::env().account_id());
        if current_allowance < amount + fee {
            return Err(FlashLenderError::AllowanceDoesNotAllowRefund)
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
    ) -> Result<(), FlashLenderError> {
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
        Ok(())
    }
}

// TODO: Refactor - FlashBorrower and FlashLender do not belong here

#[brush::wrapper]
pub type FlashBorrowerCaller = dyn FlashBorrower;

/// TODO remove eip link
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

/// TODO remove eip link
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
