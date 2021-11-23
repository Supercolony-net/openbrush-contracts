use brush::traits::{
    AccountId,
    Balance,
};
pub use common::errors::{
    FlashBorrowerError,
    FlashLenderError,
    PSP22FlashmintError,
};
use ink_prelude::vec::Vec;

// TODO: Refactor - put flash lender in different place

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
