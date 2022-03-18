pub use crate::traits::errors::{
    FlashBorrowerError,
    FlashLenderError,
};
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type FlashLenderRef = dyn FlashLender;

/// TODO remove eip link
/// Flash Lender implementation as proposed in https://eips.ethereum.org/EIPS/eip-3156)
#[brush::trait_definition]
pub trait FlashLender {
    /// Call this function in `max_flashloan` function in `impl` block of FlashLender
    /// Maximum amount of `token` available to mint
    /// Bounded by the max value of Balance (u128)
    #[ink(message)]
    fn max_flashloan(&mut self, token: AccountId) -> Balance;

    /// Call this function in `flash_fee` function in `impl` block of FlashLender
    /// Fee for borrowing `amount` of the `token`
    ///
    /// Returns `WrongTokenAddress` error if the `token` account id is not this token
    #[ink(message)]
    fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError>;

    /// Call this function in `flashloan` function in `impl` block of FlashLender
    /// Mints `amount` of `token` to `receiver_account` and performs the flashloan
    /// `amount` is then burned along with the fee for the flashloan
    /// `receiver_account` must implement `FlashBorrower`
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
    ) -> Result<(), FlashLenderError>;
}

#[brush::wrapper]
pub type FlashBorrowerRef = dyn FlashBorrower;

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
