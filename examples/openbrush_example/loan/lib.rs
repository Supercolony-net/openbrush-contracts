#![cfg_attr(not(feature = "std"), no_std)]

mod traits;

/// This contract will represent the loan of a user
#[brush::contract]
pub mod loan {
    use crate::traits::*;
    pub use crate::traits::{
        LoanInfo,
        LoanRef,
    };
    use brush::modifiers;
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ownable::traits::*;
    use psp721::{
        extensions::metadata::*,
        traits::*,
    };

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when an operator is enabled or disabled for an owner.
    /// The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    /// Define the storage for PSP721 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, PSP721Storage, OwnableStorage, PSP721MetadataStorage, LoanStorage)]
    pub struct Loan {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[OwnableStorageField]
        ownable: OwnableData,
        #[PSP721MetadataStorageField]
        metadata: PSP721MetadataData,
        #[LoanStorageField]
        loan: LoanData,
    }

    impl PSP721 for Loan {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Id) {
            self.env().emit_event(Approval { from, to, id });
        }

        fn _emit_approval_for_all_event(&self, owner: AccountId, operator: AccountId, approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner,
                operator,
                approved,
            });
        }
    }

    /// implement Ownable Trait for our NFT
    impl Ownable for Loan {}

    /// implement Metadata Trait for our NFT
    impl PSP721Metadata for Loan {}

    /// implement the storage trait of the NFT
    impl LoanTrait for Loan {
        /// We will use this function to mint new loan token and to initialize the loan's data
        #[modifiers(only_owner)]
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
        ) -> Result<(), PSP721Error> {
            let id = self._init_loan(
                borrower,
                collateral_asset,
                collateral_amount,
                borrow_asset,
                borrow_amount,
                liquidation_price,
                timestamp,
            )?;
            self._mint_to(borrower, id)
        }

        /// We will use this function to burn unused loan token and to free the loan's data
        #[modifiers(only_owner)]
        #[ink(message)]
        fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP721Error> {
            self._delete_loan(loan_id);
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
        ) -> Result<(), PSP721Error> {
            self._update_loan(loan_id, new_borrow_amount, new_timestamp, new_collateral_amount)
        }

        #[modifiers(only_owner)]
        #[ink(message)]
        fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP721Error> {
            self._liquidate_loan(loan_id)
        }
    }

    impl Loan {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_metadata(Some(String::from("Loan NFT")), Some(String::from("L-NFT")));
            instance._init_with_owner(Self::env().caller());
            instance
        }
    }
}
