pub use crate::traits::errors::OwnableError;
use brush::traits::AccountId;

#[brush::wrapper]
pub type OwnableRef = dyn Ownable;

/// Contract module which provides a basic access control mechanism, where
/// there is an account (an owner) that can be granted exclusive access to
/// specific functions.
#[brush::trait_definition]
pub trait Ownable {
    /// Returns the address of the current owner.
    #[ink(message)]
    fn owner(&self) -> AccountId;

    /// Leaves the contract without owner. It will not be possible to call
    /// owner's functions anymore. Can only be called by the current owner.
    ///
    /// NOTE: Renouncing ownership will leave the contract without an owner,
    /// thereby removing any functionality that is only available to the owner.
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner
    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;

    /// Transfers ownership of the contract to a `new_owner`.
    /// Can only be called by the current owner.
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner.
    ///
    /// Panics with `NewOwnerIsZero` error if new owner's address is zero.
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError>;
}
