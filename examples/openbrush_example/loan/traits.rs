use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
        Timestamp,
    },
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use loan_derive::LoanStorage;
use psp721::traits::Id;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LoanData {
    /// mapping from token id to the account id of borrower
    pub borrower: StorageHashMap<Id, AccountId>,
    /// mapping from token id to the account id of collateral used in loan
    pub collateral_asset: StorageHashMap<Id, AccountId>,
    /// mapping from token id to the amount of collateral deposited
    pub collateral_amount: StorageHashMap<Id, Balance>,
    /// mapping from token id to the account id of borrowed asset
    pub borrow_asset: StorageHashMap<Id, AccountId>,
    /// mapping from token id to the amount of asset borrowed
    pub borrow_amount: StorageHashMap<Id, Balance>,
    /// mapping from token id to the liquidation price of the loan
    pub liquidation_price: StorageHashMap<Id, Balance>,
    /// mapping from token id to the timestamp of when the loan was created
    pub timestamp: StorageHashMap<Id, Timestamp>,
    /// mapping from token id to the information if the loan was liquidated already or not
    pub liquidated: StorageHashMap<Id, bool>,
}

declare_storage_trait!(LoanStorage, LoanData);

// we will declare a trait which holds getters and setters for our storage struct
#[brush::trait_definition]
pub trait LoanTrait: LoanStorage {}
