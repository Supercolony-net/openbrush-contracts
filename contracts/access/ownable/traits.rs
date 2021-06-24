use brush::traits::{AccountId};
use brush::modifiers;
pub use ownable_derive::IOwnable;

#[derive(strum_macros::AsRefStr)]
pub enum OwnableError {
    CallerIsNotOwner,
    NewOwnerIsZero,
}

#[brush::trait_definition]
pub trait IOwnable {
    #[ink(message)]
    fn owner(&self) -> AccountId;

    #[ink(message)]
    #[modifiers(only_owner)]
    fn renounce_ownership(&mut self);

    #[ink(message)]
    #[modifiers(only_owner)]
    fn transfer_ownership(&mut self, new_owner: AccountId);
}
