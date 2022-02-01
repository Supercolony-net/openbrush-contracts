use brush::{
    contracts::traits::{
        ownable::*,
        psp34::{
            extensions::metadata::*,
            *,
        },
    },
    traits::{
        AccountId,
        Balance,
        Timestamp,
    },
};
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct LoanInfo {
    pub borrower: AccountId,
    pub collateral_token: AccountId,
    pub collateral_amount: Balance,
    pub borrow_token: AccountId,
    pub borrow_amount: Balance,
    pub liquidation_price: Balance,
    pub timestamp: Timestamp,
    pub liquidated: bool,
}

#[brush::wrapper]
pub type LoanRef = dyn Loan + PSP34 + PSP34Metadata + Ownable;

#[brush::trait_definition]
pub trait Loan: PSP34 + PSP34Metadata + Ownable {
    /// This function initalizes data of a loan and mint token inside it
    #[ink(message)]
    fn create_loan(&mut self, loan_info: LoanInfo) -> Result<(), PSP34Error>;

    /// This function frees data of a loan and burn token inside it
    #[ink(message)]
    fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP34Error>;

    /// This function will be used when the user repays their loan only partially
    #[ink(message)]
    fn update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP34Error>;

    /// This function will set a loan to liquidated
    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error>;

    /// Function returns `LoanInfo` by `Id`
    #[ink(message)]
    fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP34Error>;
}
