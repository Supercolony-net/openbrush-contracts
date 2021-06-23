use ink_prelude::{string::String, vec::Vec};
use brush::traits::{AccountId};
pub use psp721_derive::{IPSP721, IPSP721Metadata, IPSP721Mint, IPSP721Receiver};

pub type Id = [u8; 32];

#[derive(strum_macros::AsRefStr)]
pub enum PSP721Error {
    Unknown(String),
    CallFailed,
    NotOwner,
    NotApproved,
    TokenExists,
    TokenNotFound,
    CannotInsert,
    CannotFetchValue,
    NotAllowed,
}

#[brush::trait_definition]
pub trait IPSP721 {
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
    #[ink(message)]
    fn set_approval_for_all(&mut self, to: AccountId, approved: bool);

    /// Approves the account to transfer the specified token on behalf of the caller.
    #[ink(message)]
    fn approve(&mut self, to: AccountId, id: Id);

    /// Transfer approved or owned token.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, id: Id);

    /// Transfers token with `id` from `from` to `to`. Also some `data` can be passed.
    #[ink(message)]
    fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, data: Vec<u8>);
}

#[brush::trait_definition]
pub trait IPSP721Metadata {
    /// Returns the token name.
    #[ink(message)]
    fn name(&self) -> Option<String>;

    /// Returns the token symbol.
    #[ink(message)]
    fn symbol(&self) -> Option<String>;
}

#[brush::trait_definition]
pub trait IPSP721Mint {
    /// Creates a new token.
    #[ink(message)]
    fn mint(&mut self, id: Id);

    /// Deletes an existing token. Only the owner can burn the token.
    #[ink(message)]
    fn burn(&mut self, id: Id);
}

/// The PSP721Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP721ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

/// Handles the receipt of a single PSP-721 token type.
#[brush::trait_definition]
pub trait IPSP721Receiver {
    /// This function is called at the end of a safe_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp721_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP721ReceiverError>;
}