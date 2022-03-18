pub use crate::traits::errors::{
    PSP34Error,
    PSP34ReceiverError,
};
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;
use ink_primitives::Key;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{
    ExtKeyPtr,
    KeyPtr,
    PackedAllocate,
    PackedLayout,
    SpreadAllocate,
    SpreadLayout,
};

/// `Id` represents the identifier of the NFT. `Id::U8(1)` and `Id::U16(1)` are two different identifiers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

impl SpreadAllocate for Id {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.next_for::<Id>();
        Id::U8(0)
    }
}
impl PackedAllocate for Id {
    #[inline]
    fn allocate_packed(&mut self, _at: &Key) {}
}

#[brush::wrapper]
pub type PSP34Ref = dyn PSP34;

/// Contract module which provides a basic implementation of non fungible token.
#[brush::trait_definition]
pub trait PSP34 {
    /// Returns the collection `Id` of the NFT token.
    ///
    /// This can represents the relationship between tokens/contracts/pallets.
    #[ink(message)]
    fn collection_id(&self) -> Id;

    /// Returns the balance of the owner.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    /// Returns the owner of the token if any.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;

    /// Returns `true` if the operator is approved by the owner to withdraw `id` token.
    /// If `id` is `None`, returns `true` if the operator is approved to withdraw all owner's tokens.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    /// Approves `operator` to withdraw the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error>;

    /// Transfer approved or owned token from caller.
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
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    /// Returns current NFT total supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;
}

#[brush::wrapper]
pub type PSP34ReceiverRef = dyn PSP34Receiver;

/// PSP34Receiver is a trait for any contract that wants to support safe transfers from a PSP34
/// token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[brush::trait_definition]
pub trait PSP34Receiver {
    /// Ensures that the smart contract allows reception of PSP34 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer`
    /// or `transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP34ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34ReceiverError>;
}
