#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract will represent the loan of a user
#[brush::contract]
pub mod loan {
    use brush::contracts::{
        ownable::*,
        psp34::extensions::metadata::*,
    };

    #[cfg(not(feature = "ink-as-dependency"))]
    use brush::modifiers;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap as StorageHashMap;
    use lending_project::traits::loan::*;

    /// Define the storage for PSP34 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, PSP34Storage, OwnableStorage, PSP34MetadataStorage)]
    pub struct LoanContract {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[OwnableStorageField]
        ownable: OwnableData,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,

        // Fields of current contract
        /// mapping from token id to `LoanInfo`
        loan_info: StorageHashMap<Id, LoanInfo>,
        /// the id of last loan
        last_loan_id: Id,
        /// ids no longer used (can be reused)
        freed_ids: Vec<Id>,
    }

    /// implement PSP34 Trait for our NFT
    impl PSP34 for LoanContract {}

    /// implement Ownable Trait for our NFT
    impl Ownable for LoanContract {}

    /// implement PSP34Metadata Trait for our NFT
    impl PSP34Metadata for LoanContract {}

    impl Loan for LoanContract {
        #[modifiers(only_owner)]
        #[ink(message)]
        fn create_loan(&mut self, mut loan_info: LoanInfo) -> Result<(), PSP34Error> {
            let loan_id = self._get_next_loan_id_and_increase()?;
            if self.loan_info.get(&loan_id).is_some() {
                return Err(PSP34Error::Custom(String::from("This loan id already exists!")))
            }
            loan_info.liquidated = false;
            self.loan_info.insert(loan_id, loan_info.clone());
            self._mint_to(loan_info.borrower, loan_id)
        }

        #[modifiers(only_owner)]
        #[ink(message)]
        fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP34Error> {
            self.loan_info.take(&loan_id);
            self._burn_from(initiator, loan_id)
        }

        #[modifiers(only_owner)]
        #[ink(message)]
        fn update_loan(
            &mut self,
            loan_id: Id,
            new_borrow_amount: Balance,
            new_timestamp: Timestamp,
            new_collateral_amount: Balance,
        ) -> Result<(), PSP34Error> {
            self._update_loan(loan_id, new_borrow_amount, new_timestamp, new_collateral_amount)
        }

        #[modifiers(only_owner)]
        #[ink(message)]
        fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error> {
            self._liquidate_loan(loan_id)
        }

        #[ink(message)]
        fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP34Error> {
            let loan_info = self.loan_info.get(&loan_id);
            if loan_info.is_none() {
                return Err(PSP34Error::Custom(String::from("Loan does not exist")))
            }
            Ok(loan_info.cloned().unwrap())
        }
    }

    impl LoanContract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_metadata(Some(String::from("LoanContract NFT")), Some(String::from("L-NFT")));
            instance._init_with_owner(Self::env().caller());
            instance
        }

        /// internal function to update data of a loan
        fn _update_loan(
            &mut self,
            loan_id: Id,
            new_borrow_amount: Balance,
            new_timestamp: Timestamp,
            new_collateral_amount: Balance,
        ) -> Result<(), PSP34Error> {
            let loan_info = self.loan_info.get(&loan_id);

            if loan_info.is_none() {
                return Err(PSP34Error::Custom(String::from("This loan does not exist!")))
            }

            let mut loan_info = loan_info.cloned().unwrap();
            loan_info.collateral_amount = new_collateral_amount;
            loan_info.borrow_amount = new_borrow_amount;
            loan_info.timestamp = new_timestamp;

            self.loan_info.insert(loan_id, loan_info);

            Ok(())
        }

        /// internal function to set loan to liquidated
        fn _liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error> {
            let loan_info = self.loan_info.get(&loan_id);

            if loan_info.is_none() {
                return Err(PSP34Error::Custom(String::from("This loan does not exist!")))
            }

            let mut loan_info = loan_info.cloned().unwrap();
            loan_info.liquidated = true;

            self.loan_info.insert(loan_id, loan_info);

            Ok(())
        }

        /// internal function to return the id of a new loan and to increase it in the storage
        fn _get_next_loan_id_and_increase(&mut self) -> Result<Id, PSP34Error> {
            if self.freed_ids.len() > 0 {
                return Ok(self.freed_ids.pop().unwrap())
            }
            let mut current = self.last_loan_id;
            // It is not fully correct implementation of the increasing. but it is only an example
            for n in 0..32 {
                if current[n] == u8::MAX {
                    if n == 31 {
                        return Err(PSP34Error::Custom(String::from("Max Id reached!")))
                    } else {
                        current[n] = 0;
                    }
                } else {
                    current[n] += 1;
                    break
                }
            }
            self.last_loan_id = current;
            Ok(current)
        }
    }
}
