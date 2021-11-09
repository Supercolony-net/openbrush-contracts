use crate::{
    extensions::PSP3156FlashBorrowerStub,
    traits::{
        PSP22Error,
        PSP22,
    },
};
use brush::traits::{
    AccountId,
    Balance,
};
use ink_lang::ToAccountId;
use ink_prelude::vec::Vec;

#[brush::trait_definition]
pub trait PSP22FlashMint: PSP22 + PSP3156FlashBorrower {
    const RETURN_VALUE: [u8; 32] = brush::blake2b_256!("PSP3156FlashBorrower.onFlashLoan");

    #[ink(message)]
    fn max_flashloan(&mut self, token: AccountId) -> Balance {
        if token == Self::env().account_id() {
            Balance::MAX - self.total_supply()
        } else {
            0
        }
    }

    #[ink(message)]
    fn flash_fee(&mut self, token: AccountId, _amount: Balance) -> Balance {
        assert_eq!(
            token,
            Self::env().account_id(),
            "{}",
            PSP22Error::Custom(String::from("Wrong token")).as_ref()
        );
        0
    }

    #[ink(message)]
    fn flashloan(
        &mut self,
        receiver: &mut PSP3156FlashBorrowerStub,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let receiver_account = receiver.to_account_id();
        let fee = self.flash_fee(token, amount);
        self._mint(receiver_account, amount);
        if receiver.on_flash_loan(Self::env().caller(), token, amount, fee, data) != Self::RETURN_VALUE {
            return Err(PSP22Error::Custom(String::from("Invalid return value")))
        }
        let current_allowance = self.allowance(receiver_account, Self::env().account_id());
        if current_allowance < amount + fee {
            return Err(PSP22Error::Custom(String::from("Allowance does not allow refund")))
        }
        self._approve_from_to(
            receiver_account,
            Self::env().account_id(),
            current_allowance - amount - fee,
        )?;
        self._burn(receiver_account, amount + fee);
        Ok(())
    }
}

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
