use crate::stub::PSP22Receiver as PSP22ReceiverStub;
use ink_storage::{
    Lazy,
};
use ink_lang::ForwardCallMut;
use ink_prelude::{collections::BTreeMap, string::String, vec::Vec, format};
use ink_env::Error as EnvError;
use brush::traits::{AccountIdExt};
use brush::traits::{InkStorage, AccountId, Balance};
use brush::declare_storage_trait;
use ink_storage::{
    traits::{SpreadLayout},
};
pub use psp20_derive::{PSP22Storage, PSP22MetadataStorage};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22Data {
    pub supply: Lazy<Balance>,
    pub balances: BTreeMap<AccountId, Balance>,
    pub allowances: BTreeMap<(AccountId, AccountId), Balance>,
}

declare_storage_trait!(PSP22Storage, PSP22Data);

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP22MetadataData {
    pub name: Lazy<Option<String>>,
    pub symbol: Lazy<Option<String>>,
    pub decimals: Lazy<u8>,
}

declare_storage_trait!(PSP22MetadataStorage, PSP22MetadataData);

/// The PSP-20 error type. Contract will assert one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum PSP22Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if safe transfer check fails (see _do_safe_transfer_check() in crate::impls::PSP22)
    SafeTransferCheckFailed(String),
}

/// Trait implemented by all PSP-20 respecting smart traits.
#[brush::trait_definition]
pub trait PSP22: PSP22Storage {
    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance {
        Lazy::get(&self.get().supply).clone()
    }

    /// Returns the account Balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance {
        self.get().balances.get(&owner).copied().unwrap_or(0)
    }

    /// Transfers `value` amount of tokens from the caller's account to account `to`
    /// with additional `data` in unspecified format..
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `InsufficientBalance` error if there are not enough tokens on
    /// the caller's account Balance.
    ///
    /// Panics with `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics with `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value, data)
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set `0`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self.get().allowances.get(&(owner, spender)).copied().unwrap_or(0)
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
    /// Panics with `InsufficientAllowance` error if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    ///
    /// Panics with `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `from`.
    ///
    /// Panics with `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics with `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance, data: Vec<u8>) {
        let caller = Self::env().caller();
        let allowance = self.allowance(from, caller);
        assert!(allowance >= value, "{}", PSP22Error::InsufficientAllowance.as_ref());
        self._transfer_from_to(from, to, value, data);
        self._approve_from_to(from, caller, allowance - value);
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
    /// Panics with `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics with `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, value)
    }

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics with `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, self.allowance(owner, spender) + delta_value)
    }

    /// Atomically decreases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    ///
    /// Panics with `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics with `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) {
        let owner = Self::env().caller();
        let allowance = self.allowance(owner, spender);
        assert!(allowance >= delta_value, "{}", PSP22Error::InsufficientAllowance.as_ref());

        self._approve_from_to(owner, spender, allowance - delta_value)
    }

    // Helper functions

    /// Emit transfer event. It must be implemented in inheriting struct
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

    /// Emit approval event. It must be implemented in inheriting struct
    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    fn _do_safe_transfer_check(&self, from: AccountId, to: AccountId, value: Balance, data: Vec<u8>) {
        let mut to_receiver: PSP22ReceiverStub = ink_env::call::FromAccountId::from_account_id(to);
        match to_receiver.call_mut().before_received(Self::env().caller(), from, value, data)
            .fire()
        {
            Ok(result) => match result {
                Ok(_) => (),
                e => panic!("{}", PSP22Error::SafeTransferCheckFailed(
                    String::from(format!("The contract with `to` address does not accept tokens: {:?}", e))
                ).as_ref())
            }
            Err(e) => match e {
                EnvError::NotCallable => (),
                e => panic!("{}", PSP22Error::SafeTransferCheckFailed(
                    String::from(format!("Unknown error: call failed with {:?}", e))
                ).as_ref())
            },
        }
    }

    fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {}

    fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance, data: Vec<u8>) {
        assert!(!from.is_zero(), "{}", PSP22Error::ZeroSenderAddress.as_ref());
        assert!(!to.is_zero(), "{}", PSP22Error::ZeroRecipientAddress.as_ref());

        self._before_token_transfer(from, to, amount);

        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "{}", PSP22Error::InsufficientBalance.as_ref());
        self.get_mut().balances.insert(from, from_balance - amount);
        let to_balance = self.balance_of(to);
        self.get_mut().balances.insert(to, to_balance + amount);
        self._do_safe_transfer_check(from, to, amount, data);
        self._emit_transfer_event(Some(from), Some(to), amount);
    }

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
        assert!(!owner.is_zero(), "{}", PSP22Error::ZeroSenderAddress.as_ref());
        assert!(!spender.is_zero(), "{}", PSP22Error::ZeroRecipientAddress.as_ref());

        self.get_mut().allowances.insert((owner, spender), amount);
        self._emit_approval_event(owner, spender, amount);
    }

    /// Creates `amount` tokens and assigns them to `account`, increasing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `ZeroRecipientAddress` error if recipient's address is zero.
    fn _mint(&mut self, account: AccountId, amount: Balance) {
        assert!(!account.is_zero(), "{}", PSP22Error::ZeroRecipientAddress.as_ref());

        let mut new_balance = self.balance_of(account);
        new_balance += amount;
        self.get_mut().balances.insert(account, new_balance);
        self.get_mut().supply = Lazy::new(self.total_supply() + amount);
        self._emit_transfer_event(None, Some(account), amount);
    }

    /// Destroys `amount` tokens from `account`, reducing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `ZeroSenderAddress` error if recipient's address is zero.
    ///
    /// Panics with `InsufficientBalance` error if there are not enough tokens on
    /// the account balance of `account`.
    fn _burn(&mut self, account: AccountId, amount: Balance) {
        assert!(!account.is_zero(), "{}", PSP22Error::ZeroSenderAddress.as_ref());

        let mut from_balance = self.balance_of(account);
        assert!(from_balance >= amount, "{}", PSP22Error::InsufficientBalance.as_ref());

        from_balance -= amount;
        self.get_mut().balances.insert(account, from_balance);
        self.get_mut().supply = Lazy::new(self.total_supply() - amount);
        self._emit_transfer_event(Some(account), None, amount);
    }
}

/// Trait that contains metadata
#[brush::trait_definition]
pub trait PSP22Metadata: PSP22MetadataStorage {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String> {
        Lazy::get(&self.get().name).clone()
    }

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String> {
        Lazy::get(&self.get().symbol).clone()
    }

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8 {
        Lazy::get(&self.get().decimals).clone()
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(strum_macros::AsRefStr)]
pub enum PSP22ReceiverError {
    TransferRejected(String),
}

#[brush::trait_definition]
pub trait PSP22Receiver {
    #[ink(message)]
    fn before_received(&mut self, operator: AccountId, from: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22ReceiverError>;
}