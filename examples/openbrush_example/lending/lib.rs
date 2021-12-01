#![cfg_attr(not(feature = "std"), no_std)]

mod errors;
mod traits;

/// This will be a simple lending contract where users can:
///
/// 1. Lend tokens accepted by the smart contract.
/// The allowance and disallowance of tokens is done by the accounts which have a manager role
/// Upon lending, the user gets a PSP-22 token representing their share of the current liquidity pool
///
/// 2. Borrow tokens from the smart contract by depositing collateral tokens.
/// The tokens which can be deposited as collateral are allowed and disallowed by the accounts with manager role
/// Upon borrowing user gets a PSP-721 token representing info about their loan (how much assets were borrowed,
/// when did they borrow, what asset was borrowed, what asset was used as collateral, how much collateral assets
/// were deposited, the liquidation price of the loan and if it was liquidated or not)
///
/// 3. Repay their loan by depositing the borrowed amount of borrowed assets along with interest.
/// The contract determines how much a user needs to deposit and how much collateral they get back by an NFT token
/// which the user gets upon borrowing the assets. The user is also able to repay a portion of the loan, but will only get
/// a portion of their collateral assets back, while the liquidation price will stay the same
///
/// 4. Swap their collateral tokens to repay the borrowed amount of borrowed assets with interest.
/// The contract will perform a swap of tokens for the borrowed token on a DEX, keep the borrowed amount + interest
/// and send the rest to the user
///
/// 5. Withdraw tokens deposited to the smart contract
/// User deposits their share tokens to the smart contract and the smart contract determines how much of the underlying
/// asset they get back
///
/// 6. Liquidate a loan
/// User can call a liquidation of a loan. If the price of collateral token of the loan is below or equal to the liquidation price,
/// the loan is then liquidated and the user performing the liquidation will get 1% of the liquidated assets
///
/// 7. Allow and disallow assets for lending
/// This can only be done by the accounts with the manager role
///
/// 8. Allow and disallow assets to be used as a collateral
/// This can only be done by the accounts with the manager role
///
/// 9. Pause the contract
/// Users with the manager role can pause the contract. If the contract is paused, no borrowing or lending can be performed
/// Users can still repay their loans, liquidate loans or withdraw their deposits
#[brush::contract]
pub mod lending {
    use crate::{
        errors::*,
        traits::*,
    };
    use access_control::traits::*;
    use brush::modifiers;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use pausable::traits::*;
    use psp22::{
        extensions::mintable::*,
        traits::*,
    };
    use shares::shares::Shares;

    /// This event will be emitted when `lender` deposists `amount` of `asset` to the contract
    #[ink(event)]
    pub struct Lend {
        #[ink(topic)]
        lender: AccountId,
        #[ink(topic)]
        asset: AccountId,
        amount: Balance,
    }

    /// Define the storage for PSP22 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Default, AccessControlStorage, PausableStorage, LendingStorage)]
    pub struct Lending {
        #[AccessControlStorageField]
        access: AccessControlData,
        #[PausableStorageField]
        pause: PausableData,
        #[LendingStorageField]
        lending: LendingData,
    }

    const MANAGER: RoleType = ink_lang::selector_id!("MANAGER");

    impl LendingStorageTrait for Lending {}

    impl AccessControl for Lending {}

    impl Pausable for Lending {}

    impl Lending {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            instance.grant_role(MANAGER, caller).expect("Can not set manager role");
            instance
        }

        /// This function will allow an asset to be accepted by the contract
        /// It will also create the contracts for the shares token and lended reserves token
        #[modifiers(only_role(MANAGER))]
        #[ink(message)]
        pub fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            // we will ensure the asset is not accepted already
            if self.is_accepted_lending(asset_address) {
                return Err(LendingError::AssetSupported)
            }
            let _shares = Shares::new(Some(String::from("LendingShares")), Some(String::from("LS")));
            // create lend reserves token
            // accept it
            // self._accept_lending(asset_address, share_address: AccountId, reserve_address: AccountId)
            Ok(())
        }

        /// This function is called by a user who wants to lend tokens and gain interest
        ///
        /// `asset_address` is the AccountId of the PSP-22 token to be deposited
        /// `amount` is the amount to be deposited
        ///
        /// Returns `InsufficientAllowanceToLend` if the caller does not have enough allowance
        /// Returns `InsufficientBalanceToLend` if the caller does not have enough balance
        /// Returns `AssetNotSupported` if the asset is not supported for lending
        #[ink(message)]
        pub fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {
            // we will be using these often so we store them in variables
            let lender = Self::env().caller();
            let contract = Self::env().account_id();
            // ensure the user gave allowance to the contract
            if PSP22Wrapper::allowance(&asset_address, lender, contract) < amount {
                return Err(LendingError::InsufficientAllowanceToLend)
            }
            // ensure the user has enough assets
            if PSP22Wrapper::balance_of(&asset_address, lender) < amount {
                return Err(LendingError::InsufficientBalanceToLend)
            }
            // how much assets is already in the contract
            // if the asset is not accepted by the contract, this function will return an error
            let total_asset = self.total_asset(asset_address)?;
            // transfer the assets from user to the contract|
            PSP22Wrapper::transfer_from(&asset_address, lender, contract, amount, Vec::<u8>::new())?;
            // if no assets were deposited yet we will mint the same amount of shares as deposited `amount`
            let new_shares = if total_asset == 0 {
                amount
            } else {
                // else we calculate how much shares will belong us after depositing the `amount`
                (amount * self.total_shares(asset_address)?) / total_asset
            };
            // mint the shares token to the user
            PSP22MintableWrapper::mint(&asset_address, lender, new_shares)?;
            // emit the lend event
            self._emit_lend_event(lender, asset_address, amount);
            Ok(())
        }

        // helper functions which can only be called inside our contract

        /// helper function to emit an event when `lender` deposits `amount` of token `asset`
        fn _emit_lend_event(&self, lender: AccountId, asset: AccountId, amount: Balance) {
            self.env().emit_event(Lend { lender, asset, amount });
        }
    }
}
