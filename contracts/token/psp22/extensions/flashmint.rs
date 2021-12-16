pub use crate::{
    psp22::*,
    traits::flashloan::*,
};
use brush::traits::{
    AccountId,
    Balance,
    Flush,
};
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};

impl<T: PSP22 + PSP22Internal + Flush> FlashLender for T {
    default fn max_flashloan(&mut self, token: AccountId) -> Balance {
        if token == Self::env().account_id() {
            Balance::MAX - self.total_supply()
        } else {
            0
        }
    }

    default fn flash_fee(&mut self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
        if token != Self::env().account_id() {
            return Err(FlashLenderError::WrongTokenAddress)
        }
        Ok(self._get_fee(amount))
    }

    default fn flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let fee = self.flash_fee(token, amount)?;
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
}

pub trait PSP22FlashLenderInternal {
    fn _get_fee(&mut self, _amount: Balance) -> Balance;

    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError>;
}

impl<T: PSP22 + PSP22Internal + Flush> PSP22FlashLenderInternal for T {
    default fn _get_fee(&mut self, _amount: Balance) -> Balance {
        0
    }

    default fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        self.flush();
        let result = match FlashBorrowerRef::on_flashloan_builder(
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
                    Err(FlashBorrowerError::FlashloanRejected(message)) => {
                        Err(FlashLenderError::BorrowerRejected(message))
                    }
                }
            }
            Err(e) => {
                match e {
                    // `NotCallable` means that the receiver is not a contract.

                    // `CalleeTrapped` means that the receiver has no method called `before_received` or it failed inside.
                    // First case is expected. Second - not. But we can't tell them apart so it is a positive case for now.
                    // https://github.com/paritytech/ink/issues/1002
                    EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
                    _ => {
                        Err(FlashLenderError::BorrowerRejected(String::from(
                            "Error while performing the `on_flashloan`",
                        )))
                    }
                }
            }
        };
        self.load();
        result
    }
}
