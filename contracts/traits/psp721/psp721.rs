pub use crate::traits::errors::{
    PSP721Error,
    PSP721ReceiverError,
};
use brush::traits::AccountId;
use ink_prelude::vec::Vec;

pub type Id = [u8; 32];

#[brush::wrapper]
pub type PSP721Ref = dyn PSP721;

/// Contract module which provides a basic implementation of non fungible token.
#[brush::trait_definition]
pub trait PSP721 {
    /// Returns the balance of the owner.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    /// Returns the owner of the token.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;

    /// Returns the approved account ID for this token if any.
    #[ink(message)]
    fn get_approved(&self, id: Id) -> Option<AccountId>;

    /// Returns `true` if the operator is approved by the owner.
    #[ink(message)]
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool;

    /// Approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `ApprovalForAll` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP721Error>;

    /// Approves the account to transfer the specified token on behalf of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(&mut self, to: AccountId, id: Id) -> Result<(), PSP721Error>;

    /// Transfer approved or owned token from caller.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` is not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP721Error>;

    /// Transfer approved or owned token from `from`.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` does not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP721Error>;
}

#[brush::wrapper]
pub type PSP721ReceiverRef = dyn PSP721Receiver;

/// PSP721Receiver is a trait for any contract that wants to support safe transfers from a PSP721
/// token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[brush::trait_definition]
pub trait PSP721Receiver {
    /// Ensures that the smart contract allows reception of PSP721 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer`
    /// or `transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP721ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP721ReceiverError>;
}
