use crate::traits::Erc20Error;
pub use ink_storage::{
    collections::{
        HashMap as StorageHashMap,
    },
    Lazy,
};
use brush::{
    traits::{InkStorage, AccountId, Balance},
};
pub use ink_prelude::{string::{String}};

const ZERO_ADDRESS: [u8; 32] = [0; 32];

#[brush::internal_trait_definition]
pub trait Erc20Storage: InkStorage {
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

pub trait Erc20: Erc20Storage {
    /// Emit transfer event. It must be implemented in inherited struct
    fn emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);

    /// Emit approval event. It must be implemented in inherited struct
    fn emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

    /// Returns the token name.
    fn token_name(&self) -> Option<String> {
        Lazy::get(self._name()).clone()
    }

    /// Returns the token symbol.
    fn token_symbol(&self) -> Option<String> {
        Lazy::get(self._symbol()).clone()
    }

    /// Returns the token decimals.
    fn token_decimals(&self) -> u8 {
        Lazy::get(self._decimals()).clone()
    }

    /// Returns the total token supply.
    fn total_supply(&self) -> Balance {
        Lazy::get(self._supply()).clone()
    }

    /// Returns the account Balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    fn balance_of(&self, owner: AccountId) -> Balance {
        self._balances().get(&owner).copied().unwrap_or(0)
    }

    /// Transfers `value` amount of tokens from the caller's account to account `to`.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics `InsufficientBalance` error if there are not enough tokens on
    /// the caller's account Balance.
    ///
    /// Panics `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics `ZeroRecipientAddress` error if recipient's address is zero.
    fn transfer(&mut self, to: AccountId, value: Balance) {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value)
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set `0`.
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
    /// Panics `InsufficientAllowance` error if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    ///
    /// Panics `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `from`.
    ///
    /// Panics `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics `ZeroRecipientAddress` error if recipient's address is zero.
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) {
        let caller = Self::env().caller();
        let allowance = self.allowance(from, caller);
        assert!(allowance >= value, "{}", Erc20Error::InsufficientAllowance.as_ref());
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
    /// Panics `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics `ZeroRecipientAddress` error if recipient's address is zero.
    fn approve(&mut self, spender: AccountId, value: Balance) {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, value)
    }

    /// Sets the decimals
    fn set_decimals(&mut self, decimals: u8) {
        *self._decimals_mut() = Lazy::new(decimals);
    }

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics `ZeroRecipientAddress` error if recipient's address is zero.
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
    /// Panics `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    ///
    /// Panics `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Panics `ZeroRecipientAddress` error if recipient's address is zero.
    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) {
        let owner = Self::env().caller();
        let allowance = self.allowance(owner, spender);
        assert!(allowance >= delta_value, "{}", Erc20Error::InsufficientAllowance.as_ref());

        self._approve_from_to(owner, spender, allowance - delta_value)
    }

    /// Creates `amount` tokens and assigns them to `account`, increasing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics `ZeroRecipientAddress` error if recipient's address is zero.
    fn mint(&mut self, account: AccountId, amount: Balance) {
        assert!(account != ZERO_ADDRESS.into(), "{}", Erc20Error::ZeroRecipientAddress.as_ref());

        let mut new_balance = self.balance_of(account);
        new_balance += amount;
        self._balances_mut().insert(account, new_balance);
        *self._supply_mut() = Lazy::new(self.total_supply() + amount);
        self.emit_transfer_event(None, Some(account), amount);
    }

    /// Destroys `amount` tokens from `account`, reducing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics `ZeroSenderAddress` error if recipient's address is zero.
    ///
    /// Panics `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `account`.
    fn burn(&mut self, account: AccountId, amount: Balance) {
        assert!(account != ZERO_ADDRESS.into(), "{}", Erc20Error::ZeroSenderAddress.as_ref());

        let mut from_balance = self.balance_of(account);
        assert!(from_balance >= amount, "{}", Erc20Error::InsufficientBalance.as_ref());

        from_balance -= amount;
        self._balances_mut().insert(account, from_balance);
        *self._supply_mut() = Lazy::new(self.total_supply() - amount);
        self.emit_transfer_event(Some(account), None, amount);
    }

    // Internal functions

    fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {}

    fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
        assert!(from != ZERO_ADDRESS.into(), "{}", Erc20Error::ZeroSenderAddress.as_ref());
        assert!(to != ZERO_ADDRESS.into(), "{}", Erc20Error::ZeroRecipientAddress.as_ref());

        self._before_token_transfer(from, to, amount);

        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "{}", Erc20Error::InsufficientBalance.as_ref());
        self._balances_mut().insert(from, from_balance - amount);
        let to_balance = self.balance_of(to);
        self._balances_mut().insert(to, to_balance + amount);
        self.emit_transfer_event(Some(from), Some(to), amount);
    }

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) {
        assert!(owner != ZERO_ADDRESS.into(), "{}", Erc20Error::ZeroSenderAddress.as_ref());
        assert!(spender != ZERO_ADDRESS.into(), "{}", Erc20Error::ZeroRecipientAddress.as_ref());

        self._allowances_mut().insert((owner, spender), amount);
        self.emit_approval_event(owner, spender, amount);
    }
}