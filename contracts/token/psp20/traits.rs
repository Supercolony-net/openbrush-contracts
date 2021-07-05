pub use ink_storage::{
    collections::{
        HashMap as StorageHashMap,
    },
    Lazy,
};
pub use brush::traits::{AccountIdExt, ZERO_ADDRESS};
pub use ink_prelude::{string::{String}};
pub use ink_lang::{Env, StaticEnv};

// We don't need to expose it, because ink! will define AccountId, Balance and StaticEnv itself.
use brush::traits::{InkStorage, AccountId, Balance};

pub use psp20_derive::{PSP20Storage};

#[brush::storage_trait]
pub trait PSP20Storage: InkStorage {
    fn _supply(&self) -> & Lazy<Balance>;
    fn _supply_mut(&mut self) -> &mut Lazy<Balance>;

    fn _balances(&self) -> & StorageHashMap<AccountId, Balance>;
    fn _balances_mut(&mut self) -> &mut StorageHashMap<AccountId, Balance>;

    fn _allowances(&self) -> & StorageHashMap<(AccountId, AccountId), Balance>;
    fn _allowances_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), Balance>;

    fn _name(&self) -> & Lazy<Option<String>>;
    fn _name_mut(&mut self) -> &mut Lazy<Option<String>>;

    fn _symbol(&self) -> & Lazy<Option<String>>;
    fn _symbol_mut(&mut self) -> &mut Lazy<Option<String>>;

    fn _decimals(&self) -> & Lazy<u8>;
    fn _decimals_mut(&mut self) -> &mut Lazy<u8>;
}

/// The PSP-20 error type. Contract will assert one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum PSP20Error {
    /// Unknown error type for cases if writer of traits added own restrictions
    Unknown(&'static str),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
}

#[brush::trait_definition]
pub trait IPSP20: PSP20Storage {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String> {
        Lazy::get(self._name()).clone()
    }

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String> {
        Lazy::get(self._symbol()).clone()
    }

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8 {
        Lazy::get(self._decimals()).clone()
    }

    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance {
        Lazy::get(self._supply()).clone()
    }

    /// Returns the account Balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance {
        self._balances().get(&owner).copied().unwrap_or(0)
    }

    /// Transfers `value` amount of tokens from the caller's account to account `to`.
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
    fn transfer(&mut self, to: AccountId, value: Balance) {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value)
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set `0`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self._allowances().get(&(owner, spender)).copied().unwrap_or(0)
    }

    /// Transfers `value` tokens on the behalf of `from` to the account `to`.
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
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) {
        let caller = Self::env().caller();
        let allowance = self.allowance(from, caller);
        assert!(allowance >= value, "{}", PSP20Error::InsufficientAllowance.as_ref());
        self._transfer_from_to(from, to, value);
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
        assert!(allowance >= delta_value, "{}", PSP20Error::InsufficientAllowance.as_ref());

        self._approve_from_to(owner, spender, allowance - delta_value)
    }

    // Helper functions

    /// Emit transfer event. It must be implemented in inheriting struct
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {}

    /// Emit approval event. It must be implemented in inheriting struct
    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

    fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {}

    fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
        assert!(!from.is_zero(), "{}", PSP20Error::ZeroSenderAddress.as_ref());
        assert!(!to.is_zero(), "{}", PSP20Error::ZeroRecipientAddress.as_ref());

        self._before_token_transfer(from, to, amount);

        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "{}", PSP20Error::InsufficientBalance.as_ref());
        self._balances_mut().insert(from, from_balance - amount);
        let to_balance = self.balance_of(to);
        self._balances_mut().insert(to, to_balance + amount);
        self._emit_transfer_event(Some(from), Some(to), amount);
    }

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
        assert!(!owner.is_zero(), "{}", PSP20Error::ZeroSenderAddress.as_ref());
        assert!(!spender.is_zero(), "{}", PSP20Error::ZeroRecipientAddress.as_ref());

        self._allowances_mut().insert((owner, spender), amount);
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
        assert!(!account.is_zero(), "{}", PSP20Error::ZeroRecipientAddress.as_ref());

        let mut new_balance = self.balance_of(account);
        new_balance += amount;
        self._balances_mut().insert(account, new_balance);
        *self._supply_mut() = Lazy::new(Lazy::get(self._supply()) + amount);
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
        assert!(!account.is_zero(), "{}", PSP20Error::ZeroSenderAddress.as_ref());

        let mut from_balance = self.balance_of(account);
        assert!(from_balance >= amount, "{}", PSP20Error::InsufficientBalance.as_ref());

        from_balance -= amount;
        self._balances_mut().insert(account, from_balance);
        *self._supply_mut() = Lazy::new(Lazy::get(self._supply()) - amount);
        self._emit_transfer_event(Some(account), None, amount);
    }
}
