use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
        Timestamp,
    },
};
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use loan_derive::LoanStorage;
use psp721::traits::*;

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
    /// ids no longer used (can be reused)
    freed_ids: Vec<Id>,
}

declare_storage_trait!(LoanStorage, LoanData);

pub type LoanInfo = (
    AccountId,
    AccountId,
    Balance,
    AccountId,
    Balance,
    Balance,
    Timestamp,
    bool,
);

#[brush::wrapper]
pub type LoanRef = dyn LoanTrait + PSP721;

// we will declare a trait which holds getters and setters for our storage struct
#[brush::trait_definition]
pub trait LoanTrait: LoanStorage + PSP721 {
    /// we need to override this function and initalize data of a loan and mint token inside it
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

    /// we need to override this function and free data of a loan and burn token inside it
    #[ink(message)]
    fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP721Error>;

    /// function which returns data of a loan as a tuple
    #[ink(message)]
    fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP721Error> {
        let storage = LoanStorage::get(self);
        if storage.borrower.get(&loan_id).cloned().is_none() {
            return Err(PSP721Error::Custom(String::from("Loan does not exist")))
        }
        let borrower = storage.borrower.get(&loan_id).cloned().unwrap();
        let collateral_asset = storage.collateral_asset.get(&loan_id).cloned().unwrap();
        let collateral_amount = storage.collateral_amount.get(&loan_id).cloned().unwrap();
        let borrow_asset = storage.borrow_asset.get(&loan_id).cloned().unwrap();
        let borrow_amount = storage.borrow_amount.get(&loan_id).cloned().unwrap();
        let liquidation_price = storage.liquidation_price.get(&loan_id).cloned().unwrap();
        let timestamp = storage.timestamp.get(&loan_id).cloned().unwrap();
        let liquidated = storage.liquidated.get(&loan_id).cloned().unwrap();
        Ok((
            borrower,
            collateral_asset,
            collateral_amount,
            borrow_asset,
            borrow_amount,
            liquidation_price,
            timestamp,
            liquidated,
        ))
    }

    /// internal function to initialize the id to 0
    fn _init(&mut self) {
        let  storage = LoanStorage::get_mut(self);
        storage.last_loan_id = [0x0; 32];
        storage.freed_ids = Vec::<Id>::new();
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
        let storage = LoanStorage::get_mut(self);
        if storage.borrower.get(&loan_id).is_some() {
            return Err(PSP721Error::Custom(String::from("This loan id already exists!")))
        }
        storage.borrower.insert(loan_id, borrower);
        storage.collateral_asset.insert(loan_id, collateral_asset);
        storage.collateral_amount.insert(loan_id, collateral_amount);
        storage.borrow_asset.insert(loan_id, borrow_asset);
        storage.borrow_amount.insert(loan_id, borrow_amount);
        storage.liquidation_price.insert(loan_id, liquidation_price);
        storage.timestamp.insert(loan_id, timestamp);
        storage.liquidated.insert(loan_id, false);
        Ok(loan_id)
    }

    /// internal function to delete data of burned loan token
    fn _delete_loan(&mut self, loan_id: Id) {
        let storage = LoanStorage::get_mut(self);
        storage.borrower.take(&loan_id);
        storage.collateral_asset.take(&loan_id);
        storage.collateral_amount.take(&loan_id);
        storage.borrow_asset.take(&loan_id);
        storage.borrow_amount.take(&loan_id);
        storage.liquidation_price.take(&loan_id);
        storage.timestamp.take(&loan_id);
        storage.liquidated.take(&loan_id);
        storage.freed_ids.push(loan_id);
    }

    /// internal function to return the id of a new loan and to increase it in the storage
    fn _get_next_loan_id_and_increase(&mut self) -> Result<Id, PSP721Error> {
        let mut storage = LoanStorage::get_mut(self);
        if storage.freed_ids.len() > 0 {
            return Ok(storage.freed_ids.pop().unwrap())
        }
        let mut current = storage.last_loan_id;
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
        storage.last_loan_id = current;
        Ok(current)
    }
}
