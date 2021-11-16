use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        InkStorage,
    },
};
pub use common::errors::{
    PSP22Error,
    PSP22ReceiverError,
};
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use psp22_derive::{
    PSP22MetadataStorage,
    PSP22Storage,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22Data {
    pub supply: Balance,
    pub balances: StorageHashMap<AccountId, Balance>,
    pub allowances: StorageHashMap<(AccountId, AccountId), Balance>,
}

declare_storage_trait!(PSP22Storage, PSP22Data);

#[brush::wrapper]
pub type PSP22Wrapper = dyn PSP22;

/// Trait implemented by all PSP-20 respecting smart traits.
#[brush::trait_definition]
pub trait PSP22: PSP22Storage {
    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance {
        self.get().supply.clone()
    }

    /// Returns the account Balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance {
        self.get().balances.get(&owner).copied().unwrap_or(0)
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set `0`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self.get().allowances.get(&(owner, spender)).copied().unwrap_or(0)
    }

    /// Transfers `value` amount of tokens from the caller's account to account `to`
    /// with additional `data` in unspecified format.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the caller's account Balance.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value, data)?;
        Ok(())
    }

    /// Transfers `value` tokens on the behalf of `from` to the account `to`
    /// with additional `data` in unspecified format.
    ///
    /// This can be used to allow a contract to transfer tokens on ones behalf and/or
    /// to charge fees in sub-currencies, for example.
    ///
    /// On success a `Transfer` and `Approval` events are emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `from`.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn transfer_from(
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

    /// Allows `spender` to withdraw from the caller's account multiple times, up to
    /// the `value` amount.
    ///
    /// If this function is called again it overwrites the current allowance with `value`.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, value)?;
        Ok(())
    }

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, self.allowance(owner, spender) + delta_value)?;
        Ok(())
    }

    /// Atomically decreases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        let allowance = self.allowance(owner, spender);

        if allowance < delta_value {
            return Err(PSP22Error::InsufficientAllowance)
        }

        self._approve_from_to(owner, spender, allowance - delta_value)?;
        Ok(())
    }

    // Helper functions

    /// Emit transfer event. It must be implemented in inheriting struct
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

    /// Emit approval event. It must be implemented in inheriting struct
    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    /// Child contract can override that if they don't want to do a cross call
    fn _do_safe_transfer_check(
        &self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        match PSP22ReceiverWrapper::before_received_builder(&to, Self::env().caller(), from, value, data).fire() {
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
        }
    }

    fn _before_token_transfer(
        &mut self,
        _from: &AccountId,
        _to: &AccountId,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        Ok(())
    }

    fn _transfer_from_to(
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

        self._before_token_transfer(&from, &to, &amount)?;

        let from_balance = self.balance_of(from);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        self._do_safe_transfer_check(from, to, amount, data)?;

        self.get_mut().balances.insert(from, from_balance - amount);
        let to_balance = self.balance_of(to);
        self.get_mut().balances.insert(to, to_balance + amount);

        self._emit_transfer_event(Some(from), Some(to), amount);
        Ok(())
    }

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if owner.is_zero() {
            return Err(PSP22Error::ZeroSenderAddress)
        }
        if spender.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        self.get_mut().allowances.insert((owner, spender), amount);
        self._emit_approval_event(owner, spender, amount);
        Ok(())
    }

    /// Creates `amount` tokens and assigns them to `account`, increasing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if account.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        let mut new_balance = self.balance_of(account);
        new_balance += amount;
        self.get_mut().balances.insert(account, new_balance);
        self.get_mut().supply += amount;
        self._emit_transfer_event(None, Some(account), amount);
        Ok(())
    }

    /// Destroys `amount` tokens from `account`, reducing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if recipient's address is zero.
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the account balance of `account`.
    fn _burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if account.is_zero() {
            return Err(PSP22Error::ZeroRecipientAddress)
        }

        let mut from_balance = self.balance_of(account);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        from_balance -= amount;
        self.get_mut().balances.insert(account, from_balance);
        self.get_mut().supply -= amount;
        self._emit_transfer_event(Some(account), None, amount);
        Ok(())
    }

    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22::_burn`] and [`PSP22::allowance`].
    ///
    /// Requirements:
    ///
    /// - the caller must have allowance for `account`'s tokens of at least
    /// `amount`.
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        let current_allowance = self.allowance(account, caller);

        if current_allowance < amount {
            return Err(PSP22Error::InsufficientAllowance)
        }

        let new_amount = current_allowance - amount;
        self._approve_from_to(account, caller, new_amount)?;
        self._burn(account, amount)
    }
}

#[brush::wrapper]
pub type PSP22ReceiverWrapper = dyn PSP22Receiver;

/// PSP22Receiver is a trait for any contract that wants to support safe transfers from a PSP22
/// token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[brush::trait_definition]
pub trait PSP22Receiver {
    /// Ensures that the smart contract allows reception of PSP22 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer` or
    /// `transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP22ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22ReceiverError>;
}
