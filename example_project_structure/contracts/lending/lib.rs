#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This will be a simple lending contract where users can:
///
/// 1. Lend tokens accepted by the smart contract.
/// The allowance and disallowance of tokens is done by the accounts which have a manager role
/// Upon lending, the user gets a PSP-22 token representing their share of the current liquidity pool
///
/// 2. Borrow tokens from the smart contract by depositing collateral tokens.
/// The tokens which can be deposited as collateral are allowed and disallowed by the accounts with manager role
/// Upon borrowing user gets a PSP-34 token representing info about their loan (how much assets were borrowed,
/// when did they borrow, what asset was borrowed, what asset was used as collateral, how much collateral assets
/// were deposited, the liquidation price of the loan and if it was liquidated or not)
///
/// 3. Repay their loan by depositing the borrowed amount of borrowed assets along with interest.
/// The contract determines how much a user needs to deposit and how much collateral they get back by an NFT token
/// which the user gets upon borrowing the assets. The user is also able to repay a portion of the loan, but will only get
/// a portion of their collateral assets back, while the liquidation price will stay the same
///
/// 4. Withdraw tokens deposited to the smart contract
/// User deposits their share tokens to the smart contract and the smart contract determines how much of the underlying
/// asset they get back
///
/// 5. Liquidate a loan
/// User can call a liquidation of a loan. If the price of collateral token of the loan is below or equal to the liquidation price,
/// the loan is then liquidated and the user performing the liquidation will get 1% of the liquidated assets
///
/// 6. Allow and disallow assets for lending
/// This can only be done by the accounts with the manager role
///
/// 7. Allow and disallow assets to be used as a collateral
/// This can only be done by the accounts with the manager role
///
/// 8. Pause the contract
/// Users with the manager role can pause the contract. If the contract is paused, no borrowing or lending can be performed
/// Users can still repay their loans, liquidate loans or withdraw their deposits
#[brush::contract]
pub mod lending {
    use brush::contracts::{
        access_control::*,
        pausable::*,
    };
    use ink_lang::ToAccountId;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use lending_project::impls::lending::*;
    use loan_contract::loan::LoanContractRef;
    use shares_contract::shares::SharesContractRef;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, AccessControlStorage, PausableStorage, LendingStorage)]
    pub struct LendingContract {
        #[AccessControlStorageField]
        access: AccessControlData,
        #[PausableStorageField]
        pause: PausableData,
        #[LendingStorageField]
        lending: LendingData,
    }

    impl AccessControl for LendingContract {}

    impl Pausable for LendingContract {}

    impl Lending for LendingContract {}

    impl LendingPermissioned for LendingContract {}

    impl LendingPermissionedInternal for LendingContract {
        fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {
            let code_hash = self.lending.shares_contract_code_hash;
            let (hash, _) =
                ink_env::random::<ink_env::DefaultEnvironment>(contract_name.as_bytes()).expect("Failed to get salt");
            let hash = hash.as_ref();
            let contract =
                SharesContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
                    .endowment(0)
                    .code_hash(code_hash)
                    .salt_bytes(&hash[..4])
                    .instantiate()
                    .unwrap();
            contract.to_account_id()
        }
    }

    impl LendingContract {
        /// constructor with name and symbol
        #[ink(constructor, payable)]
        pub fn new(shares_hash: Hash, loan_hash: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut LendingContract| {
                let caller = instance.env().caller();
                instance._init_with_admin(caller);
                instance.grant_role(MANAGER, caller).expect("Can not set manager role");
                instance.lending.shares_contract_code_hash = shares_hash;
                // instantiate NFT contract and store its account id
                let nft = LoanContractRef::new()
                    .endowment(0)
                    .code_hash(loan_hash)
                    .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                    .instantiate()
                    .unwrap();
                instance.lending.loan_account = nft.to_account_id();
            })
        }
    }
}
