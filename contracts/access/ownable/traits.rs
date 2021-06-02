use ink_lang as ink;
use utils::traits::{AccountId};

#[derive(Debug, scale::Encode, scale::Decode, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
    CallerIsNotOwner,
    NewOwnerIsZero,
}

impl core::fmt::Display for OwnableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "OwnableError: {:?}", self)
    }
}

#[ink::trait_definition]
pub trait IOwnable {
    #[ink(message)]
    fn owner(&self) -> AccountId;

    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;

    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError>;
}
