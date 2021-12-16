pub use crate::traits::errors::PaymentSplitterError;
use brush::traits::{
    AccountId,
    Balance,
};

#[brush::wrapper]
pub type PaymentSplitterRef = dyn PaymentSplitter;

/// This contract allows splitting native token payments among a group of accounts. The sender does not need to be aware
/// that the native token will be split in this way, since it is handled transparently by the contract.
///
/// The split can be in equal parts or in any other arbitrary proportion. The way this is specified is by assigning each
/// account to a number of shares. Of all the native tokens that this contract receives, each account will then be able to claim
/// an amount proportional to the percentage of total shares they were assigned.
///
/// `PaymentSplitter` follows a pull payment model. This means that payments are not automatically forwarded to the
/// accounts but kept in this contract, and the actual transfer is triggered as a separate step by calling the `release`
/// function.
// TODO: Support payments in PSP22
#[brush::trait_definition]
pub trait PaymentSplitter {
    /// Getter for the total shares held by payees.
    #[ink(message)]
    fn total_shares(&self) -> Balance;

    /// Getter for the total amount of native token already released.
    #[ink(message)]
    fn total_released(&self) -> Balance;

    /// Getter for the amount of shares held by an account.
    #[ink(message)]
    fn shares(&self, account: AccountId) -> Balance;

    /// Getter for the amount of native token already released to a payee.
    #[ink(message)]
    fn released(&self, account: AccountId) -> Balance;

    /// Getter for the address of the payee number `index`.
    #[ink(message)]
    fn payee(&self, index: u32) -> AccountId;

    /// The native token received will be logged with `PaymentReceived` events.
    /// Note that these events are not fully reliable: a contract can receive a native token
    /// without triggering this function. This only affects the reliability of the events
    /// and not the actual splitting of the native token.
    ///
    /// On success a `PayeeAdded` event is emitted.
    #[ink(message, payable)]
    fn receive(&mut self);

    /// Triggers a transfer to `account` of the amount of native token they are owed, according to their percentage of the
    /// total shares and their previous withdrawals.
    ///
    /// On success a `PaymentReleased` event is emitted.
    #[ink(message)]
    fn release(&mut self, account: AccountId) -> Result<(), PaymentSplitterError>;
}
