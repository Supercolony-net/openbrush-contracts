// importing everything publicly from traits allows you to import every stuff related to lending
// by one import
pub use super::data::*;
pub use crate::traits::lending::*;
use crate::traits::{
    loan::{
        LoanInfo,
        LoanRef,
    },
    shares::SharesRef,
};
use brush::{
    contracts::{
        pausable::*,
        traits::{
            psp22::PSP22Ref,
            psp34::Id,
        },
    },
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use ink_prelude::vec::Vec;

pub const YEAR: Timestamp = 60 * 60 * 24 * 365;

impl<T: LendingStorage + PausableStorage> Lending for T {
    default fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        // get asset from mapping
        let mapped_asset = LendingStorage::get(self)
            .assets_lended
            .get(&asset_address)
            .unwrap_or(ZERO_ADDRESS.into());
        // return error if the asset is not supported
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        let contract = Self::env().account_id();
        let available = PSP22Ref::balance_of(&asset_address, contract);
        let unavailable = PSP22Ref::balance_of(&mapped_asset, contract);
        Ok(available + unavailable)
    }

    default fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        // get asset from mapping
        let mapped_asset = LendingStorage::get(self)
            .asset_shares
            .get(&asset_address)
            .unwrap_or(ZERO_ADDRESS.into());
        // return error if the asset is not supported
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        Ok(PSP22Ref::total_supply(&mapped_asset))
    }

    default fn is_accepted_lending(&self, asset_address: AccountId) -> bool {
        !LendingStorage::get(self)
            .asset_shares
            .get(&asset_address)
            .unwrap_or(ZERO_ADDRESS.into())
            .is_zero()
    }

    default fn is_accepted_collateral(&self, asset_address: AccountId) -> bool {
        LendingStorage::get(self)
            .collateral_accepted
            .get(&asset_address)
            .unwrap_or(false)
    }

    #[modifiers(when_not_paused)]
    default fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {
        // we will be using these often so we store them in variables
        let lender = Self::env().caller();
        let contract = Self::env().account_id();
        // ensure the user gave allowance to the contract
        if PSP22Ref::allowance(&asset_address, lender, contract) < amount {
            return Err(LendingError::InsufficientAllowanceToLend)
        }
        // ensure the user has enough assets
        if PSP22Ref::balance_of(&asset_address, lender) < amount {
            return Err(LendingError::InsufficientBalanceToLend)
        }
        // how much assets is already in the contract
        // if the asset is not accepted by the contract, this function will return an error
        let total_asset = self.total_asset(asset_address)?;
        // transfer the assets from user to the contract|
        PSP22Ref::transfer_from(&asset_address, lender, contract, amount, Vec::<u8>::new())?;
        // if no assets were deposited yet we will mint the same amount of shares as deposited `amount`
        let new_shares = if total_asset == 0 {
            amount
        } else {
            // else we calculate how much shares will belong us after depositing the `amount`
            (amount * self.total_shares(asset_address)?) / total_asset
        };
        let reserve_asset = get_reserve_asset(self, &asset_address)?;
        // mint the shares token to the user
        SharesRef::mint(&reserve_asset, lender, new_shares)?;
        Ok(())
    }

    #[modifiers(when_not_paused)]
    default fn borrow_assets(
        &mut self,
        asset_address: AccountId,
        collateral_address: AccountId,
        amount: Balance,
    ) -> Result<(), LendingError> {
        // we will be using these often so we store them in variables
        let borrower = Self::env().caller();
        let contract = Self::env().account_id();
        // ensure this asset is accepted as collateral
        if !self.is_accepted_collateral(collateral_address) {
            return Err(LendingError::AssetNotSupported)
        }
        // ensure the user gave allowance to the contract
        if PSP22Ref::allowance(&collateral_address, borrower, contract) < amount {
            return Err(LendingError::InsufficientAllowanceForCollateral)
        }
        // ensure the user has enough collateral assets
        if PSP22Ref::balance_of(&collateral_address, borrower) < amount {
            return Err(LendingError::InsufficientCollateralBalance)
        }
        let reserve_asset = get_reserve_asset(self, &asset_address)?;

        // we will find out the price of deposited collateral
        let price = get_asset_price(self, amount, collateral_address, asset_address);
        // we will set the liquidation price to be 75% of current price
        let liquidation_price = (price * 75) / 100;
        // borrow amount is 70% of collateral
        let borrow_amount = (price * 70) / 100;
        // ensure the liquidation price is greater than borrowed amount to avoid misuses
        if borrow_amount >= liquidation_price {
            return Err(LendingError::AmountNotSupported)
        }
        // ensure we have enough assets in the contract
        if PSP22Ref::balance_of(&asset_address, contract) < borrow_amount {
            return Err(LendingError::InsufficientBalanceInContract)
        }
        // we will transfer the collateral to the contract
        PSP22Ref::transfer_from(&collateral_address, borrower, contract, amount, Vec::<u8>::new())?;
        // create loan info
        let loan_info = LoanInfo {
            borrower,
            collateral_token: collateral_address,
            collateral_amount: amount,
            borrow_token: asset_address,
            borrow_amount,
            liquidation_price,
            timestamp: Self::env().block_timestamp(),
            liquidated: false,
        };

        let load_account = LendingStorage::get(self).loan_account;
        LoanRef::create_loan(&load_account, loan_info)?;
        // transfer assets to borrower
        PSP22Ref::transfer(&asset_address, borrower, borrow_amount, Vec::<u8>::new())?;
        // mint `borrow_amount` of the reserve token
        SharesRef::mint(&reserve_asset, contract, borrow_amount)?;
        Ok(())
    }

    default fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, LendingError> {
        // REPAYING (borrower: B, nft, repayAmount: X):
        let initiator = Self::env().caller();
        let contract = Self::env().account_id();
        let loan_account = LendingStorage::get(self).loan_account;
        let apy = 1000;
        // initiator must own the nft
        if LoanRef::owner_of(&loan_account, loan_id.clone()).unwrap_or(ZERO_ADDRESS.into()) != initiator {
            return Err(LendingError::NotTheOwner)
        }
        let loan_info = LoanRef::get_loan_info(&loan_account, loan_id.clone())?;
        if loan_info.liquidated {
            LoanRef::delete_loan(&loan_account, initiator, loan_id.clone())?;
            return Ok(false)
        }

        // ensure initiator has enough allowance
        if PSP22Ref::allowance(&loan_info.borrow_token, initiator, contract) < repay_amount {
            return Err(LendingError::InsufficientAllowanceToRepay)
        }
        // ensure initiator has enough balance
        if PSP22Ref::balance_of(&loan_info.borrow_token, initiator) < repay_amount {
            return Err(LendingError::InsufficientBalanceToRepay)
        }
        let time_passed = Self::env().block_timestamp() - loan_info.timestamp;
        let total_apy = (apy * time_passed as Balance) / YEAR as Balance;
        let to_repay = (((loan_info.borrow_amount) * (10000 + total_apy)) / 10000) + 1;
        let reserve_asset = get_reserve_asset(self, &loan_info.borrow_token)?;
        if repay_amount >= to_repay {
            PSP22Ref::transfer_from(&loan_info.borrow_token, initiator, contract, to_repay, Vec::<u8>::new())?;
            PSP22Ref::transfer(
                &loan_info.collateral_token,
                initiator,
                loan_info.collateral_amount,
                Vec::<u8>::new(),
            )?;
            LoanRef::delete_loan(&loan_account, initiator, loan_id)?;
            SharesRef::burn(&reserve_asset, Self::env().caller(), loan_info.borrow_amount)?;
        } else {
            PSP22Ref::transfer_from(
                &loan_info.borrow_token,
                initiator,
                contract,
                repay_amount,
                Vec::<u8>::new(),
            )?;
            let to_return = (repay_amount * loan_info.collateral_amount) / to_repay;
            PSP22Ref::transfer(&loan_info.collateral_token, initiator, to_return, Vec::<u8>::new())?;
            SharesRef::mint(
                &reserve_asset,
                contract,
                to_repay - repay_amount - loan_info.borrow_amount,
            )?;
            LoanRef::update_loan(
                &loan_account,
                loan_id.clone(),
                to_repay - repay_amount,
                Self::env().block_timestamp(),
                loan_info.collateral_amount - to_return,
            )?;
        }
        Ok(true)
    }

    default fn withdraw_asset(
        &mut self,
        shares_address: AccountId,
        shares_amount: Balance,
    ) -> Result<(), LendingError> {
        let withdraw_asset = get_asset_from_shares(self, shares_address)?;
        let withdraw_amount =
            (shares_amount * self.total_asset(withdraw_asset)?) / PSP22Ref::total_supply(&shares_address);
        if withdraw_amount > PSP22Ref::balance_of(&withdraw_asset, Self::env().account_id()) {
            return Err(LendingError::InsufficientBalanceInContract)
        }

        SharesRef::burn(&shares_address, Self::env().caller(), shares_amount)?;
        PSP22Ref::transfer(&withdraw_asset, Self::env().caller(), withdraw_amount, Vec::<u8>::new())?;
        Ok(())
    }

    default fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), LendingError> {
        let loan_account = LendingStorage::get(self).loan_account;
        let loan_info = LoanRef::get_loan_info(&loan_account, loan_id.clone())?;

        if loan_info.liquidated {
            return Err(LendingError::LoanLiquidated)
        }

        let price = get_asset_price(
            self,
            loan_info.collateral_amount,
            loan_info.collateral_token,
            loan_info.borrow_token,
        );

        if price <= loan_info.liquidation_price {
            // if we swapped the collateral to borrow asset we would burn the reserve tokens
            // let reserve_asset = self._get_reserve_asset(borrow_asset);
            // PSP22BurnableRef::burn(&reserve_asset, borrow_amount)
            let reward = (loan_info.collateral_amount * 1000) / 100000;
            PSP22Ref::transfer(
                &loan_info.collateral_token,
                Self::env().caller(),
                reward,
                Vec::<u8>::new(),
            )?;
            LoanRef::liquidate_loan(&loan_account, loan_id.clone())?;
        } else {
            return Err(LendingError::CanNotBeLiquidated)
        }
        Ok(())
    }
}
