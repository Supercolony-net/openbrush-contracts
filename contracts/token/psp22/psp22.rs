pub use crate::traits::{
    errors::{
        PSP22Error,
        PSP22ReceiverError,
    },
    psp22::*,
};
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Flush,
    },
};
pub use derive::{
    PSP22MetadataStorage,
    PSP22Storage,
};
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout, SpreadAllocate)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22Data {
    pub supply: Balance,
    pub balances: Mapping<AccountId, Balance>,
    pub allowances: Mapping<(AccountId, AccountId), Balance>,
}

declare_storage_trait!(PSP22Storage, PSP22Data);

impl<T: PSP22Storage + Flush> PSP22 for T {
    default fn total_supply(&self) -> Balance {
        self.get().supply.clone()
    }

    default fn balance_of(&self, owner: AccountId) -> Balance {
        self.get().balances.get(&owner).unwrap_or(0)
    }

    default fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self.get().allowances.get(&(owner, spender)).unwrap_or(0)
    }

    default fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value, data)?;
        Ok(())
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let allowance = self.allowance(from, caller);

        if allowance < value {
            return Err(PSP22Error::InsufficientAllowance)
        }

        self._transfer_from_to(from, to, value, data)?;
        self._approve_from_to(from, caller, allowance - value)?;
        Ok(())
    }

    default fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, value)?;
        Ok(())
    }

    default fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, self.allowance(owner, spender) + delta_value)?;
        Ok(())
    }

    default fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        let allowance = self.allowance(owner, spender);

        if allowance < delta_value {
            return Err(PSP22Error::InsufficientAllowance)
        }

        self._approve_from_to(owner, spender, allowance - delta_value)?;
        Ok(())
    }
}

pub trait PSP22Internal {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

    fn _do_safe_transfer_check(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error>;

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error>;

    fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}

impl<T: PSP22Storage + Flush> PSP22Internal for T {
    default fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

    default fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    default fn _do_safe_transfer_check(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        self.flush();
        let result =
            match PSP22ReceiverRef::before_received_builder(&to, Self::env().caller(), from, value, data).fire() {
                Ok(result) => {
                    match result {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.into()),
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
                            Err(PSP22Error::SafeTransferCheckFailed(String::from(
                                "Error during call to receiver",
                            )))
                        }
                    }
                }
            };
        self.load();
        result?;
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        Ok(())
    }

    default fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        if from.is_zero() {
            return Err(PSP22Error::ZeroSenderAddress)
        }
        if to.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        let from_balance = self.balance_of(from);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        self._before_token_transfer(Some(&from), Some(&to), &amount)?;

        self._do_safe_transfer_check(from, to, amount, data)?;
        self.get_mut().balances.insert(from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.get_mut().balances.insert(to, &(to_balance + amount));

        self._emit_transfer_event(Some(from), Some(to), amount);
        self._after_token_transfer(Some(&from), Some(&to), &amount)
    }

    default fn _approve_from_to(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
    ) -> Result<(), PSP22Error> {
        if owner.is_zero() {
            return Err(PSP22Error::ZeroSenderAddress)
        }
        if spender.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        self.get_mut().allowances.insert((owner, spender), &amount);
        self._emit_approval_event(owner, spender, amount);
        Ok(())
    }

    default fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if account.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        self._before_token_transfer(None, Some(&account), &amount)?;
        let mut new_balance = self.balance_of(account);
        new_balance += amount;
        self.get_mut().balances.insert(account, &new_balance);
        self.get_mut().supply += amount;
        self._emit_transfer_event(None, Some(account), amount);
        self._after_token_transfer(None, Some(&account), &amount)
    }

    default fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if account.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        let mut from_balance = self.balance_of(account);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        self._before_token_transfer(Some(&account), None, &amount)?;

        from_balance -= amount;
        self.get_mut().balances.insert(account, &from_balance);
        self.get_mut().supply -= amount;
        self._emit_transfer_event(Some(account), None, amount);

        self._after_token_transfer(Some(&account), None, &amount)
    }
}
