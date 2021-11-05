use crate::traits::PSP22;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::trait_definition]
pub trait PSP22FlashMint: PSP22 + PSP3156FlashBorrower {}

#[brush::trait_definition]
pub trait PSP3156FlashBorrower {
    #[ink(message)]
    fn on_flash_loan(
        &mut self,
        initiator: AccountId,
        token: AccountId,
        amount: Balance,
        fee: Balance,
        data: Vec<u8>,
    ) -> [u8; 32];
}
