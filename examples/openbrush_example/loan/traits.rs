use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
        Timestamp,
    },
};
use ink_prelude::string::String;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use loan_derive::LoanStorage;
use psp721::traits::{
    Id,
    PSP721Error,
};

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
    /// the id of last loan
    last_loan_id: Id,
}

declare_storage_trait!(LoanStorage, LoanData);

// we will declare a trait which holds getters and setters for our storage struct
#[brush::trait_definition]
pub trait LoanTrait: LoanStorage {
    /// internal function to initialize the id to 0
    fn _init(&mut self) {
        self.get_mut().last_loan_id = [0x0; 32];
    }

    /// internal function to initialize data of a new loan token
    fn _init_loan(
        &mut self,
        borrower: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        borrow_asset: AccountId,
        borrow_amount: Balance,
        liquidation_price: Balance,
        timestamp: Timestamp,
    ) -> Result<Id, PSP721Error> {
        let loan_id = self._get_next_loan_id_and_increase()?;
        if self.get_mut().borrower.get(&loan_id).is_some() {
            return Err(PSP721Error::Custom(String::from("This loan id already exists!")))
        }
        self.get_mut().borrower.insert(loan_id, borrower);
        self.get_mut().collateral_asset.insert(loan_id, collateral_asset);
        self.get_mut().collateral_amount.insert(loan_id, collateral_amount);
        self.get_mut().borrow_asset.insert(loan_id, borrow_asset);
        self.get_mut().borrow_amount.insert(loan_id, borrow_amount);
        self.get_mut().liquidation_price.insert(loan_id, liquidation_price);
        self.get_mut().timestamp.insert(loan_id, timestamp);
        self.get_mut().liquidated.insert(loan_id, false);
        Ok(loan_id)
    }

    /// internal function to return the id of a new loan and to increase it in the storage
    fn _get_next_loan_id_and_increase(&mut self) -> Result<Id, PSP721Error> {
        let mut current = self.get_mut().last_loan_id;
        for n in 0..32 {
            if current[n] == u8::MAX {
                if n == 31 {
                    return Err(PSP721Error::Custom(String::from("Max Id reached!")))
                } else {
                    current[n] = 0;
                }
            } else {
                current[n] += 1;
                break
            }
        }
        self.get_mut().last_loan_id = current;
        Ok(current)
    }
}

#[brush::wrapper]
pub type LoanRef = dyn LoanContract;

/// We will add this trait and implement it in our loan contract so we can refer to its method in other contracts
#[brush::trait_definition]
pub trait LoanContract {
    #[ink(message)]
    fn create_loan(
        &mut self,
        borrower: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        borrow_asset: AccountId,
        borrow_amount: Balance,
        liquidation_price: Balance,
        timestamp: Timestamp,
    ) -> Result<(), PSP721Error>;
}
