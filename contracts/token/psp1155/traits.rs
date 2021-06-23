use core::result::Result;
use ink_prelude::{
    string::{String},
    vec::Vec,
};
use brush::traits::{AccountId, Balance};

pub type Id = [u8; 32];

#[derive(strum_macros::AsRefStr)]
pub enum PSP1155Error {
    Unknown(String),
    CallFailed,
    ZeroAddress,
    SelfApproval,
    InsufficientBalance,
    MaxBalance,
    TransferToZeroAddress,
    ApproveRequired,
    InputLengthMismatch,
}

/// A standard trait for contracts that manage multiple token types.
/// A single deployed contract may include any combination of fungible tokens,
/// non-fungible tokens or other configurations (e.g. semi-fungible tokens).
#[brush::trait_definition]
pub trait IPSP1155 {
    /// Returns the amount of tokens of token type `_id` owned by `_account`.
    #[ink(message)]
    fn balance_of(&self, _account: AccountId, _id: Id) -> Balance;

    /// Batched version of {balance_of}.
    #[ink(message)]
    fn balance_of_batch(&self, _owners: Vec<AccountId>, _ids: Vec<Id>) -> Vec<Balance>;

    /// Grants or revokes permission to `_operator` to transfer the caller's tokens, according to `_approved`
    #[ink(message)]
    fn set_approval_for_all(&mut self, _operator: AccountId, _approved: bool);

    /// Returns true if `_operator` is approved to transfer ``_account``'s tokens.
    #[ink(message)]
    fn is_approved_for_all(&self, _account: AccountId, _operator: AccountId) -> bool;

    /// Transfers `_amount` tokens of token type `_id` from `_from` to `_to`. Also some `_data` can be passed.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _id: Id,
        _amount: Balance,
        _data: Vec<u8>,
    );

    /// Batched version of {safe_transfer_from}.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        _from: AccountId,
        _to: AccountId,
        _ids: Vec<Id>,
        _amounts: Vec<Balance>,
        _data: Vec<u8>,
    );
}

#[brush::trait_definition]
pub trait IPSP1155Mint {
    /// Creates a new token.
    #[ink(message)]
    fn mint(&mut self, to: AccountId, id: Id, amount: Balance);

    /// Deletes an existing token. Only the owner can burn the token.
    #[ink(message)]
    fn burn(&mut self, from: AccountId, id: Id, amount: Balance);
}

#[brush::trait_definition]
pub trait IPSP1155MetadataURI {
    /// Returns the URI for token type `id`.
    #[ink(message)]
    fn uri(&self, _id: Id) -> Option<String>;
}

/// The PSP1155Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP1155ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected,
}

/// Handles the receipt of a single PSP-1155 token type.
#[brush::trait_definition]
pub trait IPSP1155Receiver {
    /// This function is called at the end of a safe_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp1155_received(&mut self, _operator: AccountId, _from: AccountId,
                           _id: Id, _value: Balance, _data: Vec<u8>) -> Result<(), PSP1155ReceiverError>;

    /// This function is called at the end of a safe_batch_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp1155_batch_received(&mut self, _operator: AccountId, _from: AccountId,
                                 _ids: Vec<Id>, _values: Vec<Balance>, _data: Vec<u8>) -> Result<(), PSP1155ReceiverError>;
}
