use ink_lang as ink;
use ink_prelude::{string::String};
use utils::traits::{AccountId, Balance};

/// The ERC-20 error type.
#[derive(Debug, scale::Encode, scale::Decode, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Erc20Error {
    /// Unknown error type for cases if writer of traits added own restrictions
    Unknown(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
}

impl core::fmt::Display for Erc20Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Erc20Error: {:?}", self)
    }
}

/// Trait implemented by all ERC-20 respecting smart traits.
#[ink::trait_definition]
pub trait IErc20 {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String>;

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String>;

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8;

    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    /// Returns the account balance for the specified `owner`.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    /// Transfers `value` amount of tokens from the caller's account to account `to`.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Erc20Error>;

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    /// Transfers `value` tokens on the behalf of `from` to the account `to`.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error>;

    /// Allows `spender` to withdraw from the caller's account multiple times, up to
    /// the `value` amount.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Erc20Error>;

    /// Atomically increases the allowance granted to `spender` by the caller on `delta_value`.
    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error>;

    /// Atomically decreases the allowance granted to `spender` by the caller on `delta_value`.
    #[ink(message)]
    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error>;
}