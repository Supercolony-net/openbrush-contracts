pub use crate::traits::errors::{
    PSP1155Error,
    PSP1155ReceiverError,
};
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

pub type Id = [u8; 32];

#[brush::wrapper]
pub type PSP1155Ref = dyn PSP1155;

/// Contract module which provides a basic implementation of multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
#[brush::trait_definition]
pub trait PSP1155 {
    /// Returns the amount of tokens of token type `id` owned by `account`.
    #[ink(message)]
    fn balance_of(&self, account: AccountId, id: Id) -> Balance;

    /// Batched version of {balance_of}.
    #[ink(message)]
    fn balance_of_batch(&self, accounts_ids: Vec<(AccountId, Id)>) -> Vec<Balance>;

    /// Grants or revokes permission to `operator` to transfer the caller's tokens, according to `approved`
    ///
    /// On success a `ApprovalForAll` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `NotAllowed` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP1155Error>;

    /// Returns true if `operator` is approved to transfer ``account``'s tokens.
    #[ink(message)]
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool;

    /// Transfers `amount` tokens of token type `id` from `from` to `to`. Also some `data` can be passed.
    ///
    /// On success a `TransferSingle` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TransferToZeroAddress` error if recipient is zero account.
    ///
    /// Returns `NotAllowed` error if transfer is not approved.
    ///
    /// Returns `InsufficientBalance` error if `from` doesn't contain enough balance.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error>;

    /// Batched version of {safe_transfer_from}.
    ///
    /// On success a `TransferBatch` event is emitted.
    #[ink(message)]
    fn batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155Error>;
}

#[brush::wrapper]
pub type PSP1155ReceiverRef = dyn PSP1155Receiver;

/// PSP1155Receiver is a trait for any contract that wants to support safe transfers from a PSP1155
/// multi token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[brush::trait_definition]
pub trait PSP1155Receiver {
    /// Ensures that the smart contract allows reception of PSP1155 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer_from`
    /// or `batch_transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP1155ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        ids_to_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP1155ReceiverError>;
}
