use brush::traits::{AccountId};

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
    fn renounce_ownership(&mut self);

    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: AccountId);
}
