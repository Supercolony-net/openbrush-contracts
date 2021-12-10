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
    use crate::{
        errors::*,
        traits::*,
    };
    use access_control::traits::*;
    use brush::{
        modifiers,
        traits::{
            AccountIdExt,
            ZERO_ADDRESS,
        },
    };
    use ink_lang::ToAccountId;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use loan_nft::loan::{
        Loan,
        LoanRef,
    };
    use pausable::traits::*;
    use psp22::{
        extensions::{
            burnable::*,
            mintable::*,
        },
        traits::*,
    };
    use psp721::traits::Id;
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

    /// This event will be emitted when `manager_address` accepts `asset_address` for lending
    #[ink(event)]
    pub struct LendingAllowed {
        #[ink(topic)]
        asset_address: AccountId,
        #[ink(topic)]
        shares_address: AccountId,
        #[ink(topic)]
        reserves_address: AccountId,
        #[ink(topic)]
        manager_address: AccountId,
    }

    /// This event will be emitted when `manager_address` cancels `asset_address` for lending
    #[ink(event)]
    pub struct LendingCancelled {
        #[ink(topic)]
        asset_address: AccountId,
        #[ink(topic)]
        manager_address: AccountId,
    }

    /// This event will be emitted when `manager_address` accepts `asset_address` for being used as collateral
    #[ink(event)]
    pub struct CollateralAllowed {
        #[ink(topic)]
        asset_address: AccountId,
        #[ink(topic)]
        manager_address: AccountId,
    }

    /// This event will be emitted when `manager_address` cancels usage of `asset_address` as collateral
    #[ink(event)]
    pub struct CollateralCancelled {
        #[ink(topic)]
        asset_address: AccountId,
        #[ink(topic)]
        manager_address: AccountId,
    }

    /// This event will be emitted when `borrower` borrows `borrow_amount` of `asset_address`
    /// while depositing `collateral_amount` of `collateral_address` as collateral
    #[ink(event)]
    pub struct Borrow {
        #[ink(topic)]
        borrower: AccountId,
        #[ink(topic)]
        collateral_address: AccountId,
        #[ink(topic)]
        asset_address: AccountId,
        collateral_amount: Balance,
        borrow_amount: Balance,
    }

    /// This event will be emitted when `borrower` repays `repay_amount` of `asset_address`
    /// while getting back `collateral_amount` of `collateral_address`
    #[ink(event)]
    pub struct Repay {
        #[ink(topic)]
        borrower: AccountId,
        #[ink(topic)]
        collateral_address: AccountId,
        #[ink(topic)]
        asset_address: AccountId,
        collateral_amount: Balance,
        repay_amount: Balance,
        to_repay: Balance,
    }

    /// This event will be emitted when `lender` withdraws `withdraw_amount` of `asset_address`
    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        lender: AccountId,
        #[ink(topic)]
        asset_address: AccountId,
        withdraw_amount: Balance,
    }

    /// This event will be emitted when `liquidator` liquidates a loan of `borrower`
    /// liquidating `collateral_amount` of `collateral_address` while getting `liquidator_fee`
    #[ink(event)]
    pub struct Liquidate {
        #[ink(topic)]
        liquidator: AccountId,
        #[ink(topic)]
        borrower: AccountId,
        #[ink(topic)]
        collateral_address: AccountId,
        collateral_amount: Balance,
        liquidator_fee: Balance,
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
        code_hash: Hash,
        nft_contract: AccountId,
    }

    const MANAGER: RoleType = ink_lang::selector_id!("MANAGER");
    const YEAR: Timestamp = 60 * 60 * 24 * 365;

    impl LendingStorageTrait for Lending {}

    impl AccessControl for Lending {}

    impl Pausable for Lending {}

    impl Lending {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new(code_hash: Hash, nft_code_hash: Hash) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            instance.grant_role(MANAGER, caller).expect("Can not set manager role");
            instance.code_hash = code_hash;
            // instantiate NFT contract and store its account id
            let nft = Loan::new()
                .endowment(25)
                .code_hash(nft_code_hash)
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate()
                .unwrap();
            instance.nft_contract = nft.to_account_id();
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
            // instantiate the shares of the lended assets
            let shares_address = self._instantiate_shares_contract("LendingShares", "LS");
            // instantiate the reserves of the borrowed assets
            let reserves_address = self._instantiate_shares_contract("LendingReserves", "LR");
            // accept the asset and map shares and reserves to it
            self._accept_lending(asset_address, shares_address, reserves_address);
            self._emit_lending_accepted_event(asset_address, shares_address, reserves_address, self.env().caller());
            Ok(())
        }

        /// This function will disallow lending and borrowing of asset
        /// To do this all assets of this asset must be repaid and all of the asset must be withdrawn
        #[modifiers(only_role(MANAGER))]
        #[ink(message)]
        pub fn disallow_lending(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            let reserve_asset = self._get_reserve_asset(asset_address);
            if PSP22Ref::balance_of(&asset_address, Self::env().account_id()) > 0
                || PSP22Ref::balance_of(&reserve_asset, Self::env().account_id()) > 0
            {
                return Err(LendingError::AssetsInTheContract)
            }
            self._disallow_lending(asset_address);
            self._emit_lending_cancelled_event(asset_address, Self::env().caller());
            Ok(())
        }

        /// This function will allow an asset to be accepted as collateral
        #[modifiers(only_role(MANAGER))]
        #[ink(message)]
        pub fn allow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            // we will ensure the asset is not accepted already
            if self.is_accepted_collateral(asset_address) {
                return Err(LendingError::AssetSupported)
            }
            self._set_collateral_accepted(asset_address, true);
            self._emit_collateral_accepted_event(asset_address, Self::env().caller());
            Ok(())
        }

        /// This function will disallow an asset to be accepted as collateral
        #[modifiers(only_role(MANAGER))]
        #[ink(message)]
        pub fn disallow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
            // we will ensure the asset is not accepted already
            if self.is_accepted_collateral(asset_address) {
                self._set_collateral_accepted(asset_address, false);
                self._emit_collateral_cancelled_event(asset_address, Self::env().caller());
            }
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
        #[modifiers(when_not_paused)]
        #[ink(message)]
        pub fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {
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
            // mint the shares token to the user
            PSP22MintableRef::mint(&asset_address, lender, new_shares)?;
            // emit the lend event
            self._emit_lend_event(lender, asset_address, amount);
            Ok(())
        }

        /// This function is called by a user who wants to borrow tokens. In order to do that,
        /// they need to deposit collateral. The value of borrowed assets will be equal to 70%
        /// of the value of deposited collateral.
        ///
        /// `asset_address` is the AccountId of the PSP-22 token to be borrowed
        /// `collateral_address` is the AccountId of the PSP-22 token used as collateral
        /// `amount` is the amount to be deposited
        ///
        /// Returns `AssetNotSupported` if `asset_address` is not supported for using as collateral
        /// Returns `InsufficientAllowanceForCollateral` if the caller does not have enough allowance
        /// Returns `InsufficientCollateralBalance` if the caller does not have enough balance
        /// Returns `AssetNotSupported` if the borrowing asset is not supported for borrowing
        /// Returns `AmountNotSupported` if the liquidation price is less than or equal to the borrowed amount
        /// Returns `InsufficientBalanceInContract` if there is not enough amount of assets in the contract to borrow
        #[modifiers(when_not_paused)]
        #[ink(message)]
        pub fn borrow_assets(
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
            let reserve_asset = self._get_reserve_asset(asset_address);
            // ensure the asset is supported by our contract
            if reserve_asset.is_zero() {
                return Err(LendingError::AssetNotSupported)
            }
            // we will find out the price of deposited collateral
            let price = self._price_of(amount, collateral_address, asset_address);
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
            // create loan nft
            let nft_address = self.nft_contract;
            LoanRef::create_loan(
                &nft_address,
                borrower,
                collateral_address,
                amount,
                asset_address,
                borrow_amount,
                liquidation_price,
                Self::env().block_timestamp(),
            )?;
            // transfer assets to borrower
            PSP22Ref::transfer(&asset_address, borrower, borrow_amount, Vec::<u8>::new())?;
            // mint `borrow_amount` of the reserve token
            PSP22MintableRef::mint(&reserve_asset, contract, borrow_amount)?;
            self._emit_borrow_event(borrower, collateral_address, asset_address, amount, borrow_amount);
            Ok(())
        }

        /// This function is called by the user who borrowed some asset. User needs to deposit borrowed amount along with interest
        /// They can either repay the full amount or just a portion of the amount. If they repay the full amount, they will get all deposited
        /// collateral back, another way they will get back the same portion of collateral as the repay portion (eg. if they deposit 80% of
        /// the loan + interests, they will get 80% of collateral back). If the loan was liquidated, the user does not get their collateral
        /// back and the NFT will be burned
        ///
        /// `loan_id` is the id of the loan to be repaid
        /// `repay_amount` is the amount of borrowed asset to be repaid
        ///
        /// Returns true if the loan was repaid successfuly, false if the loan was already liquidated and can not be repaid
        /// Returns `NotTheOwner` error if the initiator is not the owner of the loan token
        /// Returns `InsufficientAllowanceToRepay` error if the initiator did not give allowance to the contract
        /// Returns `InsufficientBalanceToRepay` error if the initiator tries to repay more tokens than their balance
        #[ink(message)]
        pub fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, LendingError> {
            // REPAYING (borrower: B, nft, repayAmount: X):
            let initiator = Self::env().caller();
            let contract = Self::env().account_id();
            let loan_contract = self.nft_contract;
            let apy = 1000;
            // initiator must own the nft
            if LoanRef::owner_of(&loan_contract, loan_id).unwrap_or(ZERO_ADDRESS.into()) != initiator {
                return Err(LendingError::NotTheOwner)
            }
            let loan_info = LoanRef::get_loan_info(&loan_contract, loan_id)?;
            if loan_info.7 {
                LoanRef::delete_loan(&loan_contract, initiator, loan_id)?;
                return Ok(false)
            }
            let collateral_asset = loan_info.1;
            let collateral_amount = loan_info.2;
            let borrow_asset = loan_info.3;
            let borrow_amount = loan_info.4;
            // ensure initiator has enough allowance
            if PSP22Ref::allowance(&borrow_asset, initiator, contract) < repay_amount {
                return Err(LendingError::InsufficientAllowanceToRepay)
            }
            // ensure initiator has enough balance
            if PSP22Ref::balance_of(&borrow_asset, initiator) < repay_amount {
                return Err(LendingError::InsufficientBalanceToRepay)
            }
            let time_passed = Self::env().block_timestamp() - loan_info.6;
            let total_apy = (apy * time_passed as Balance) / YEAR as Balance;
            let to_repay = (((borrow_amount) * (10000 + total_apy)) / 10000) + 1;
            let reserve_asset = self._get_reserve_asset(borrow_asset);
            if repay_amount >= to_repay {
                PSP22Ref::transfer_from(&borrow_asset, initiator, contract, to_repay, Vec::<u8>::new())?;
                PSP22Ref::transfer(&collateral_asset, initiator, collateral_amount, Vec::<u8>::new())?;
                LoanRef::delete_loan(&loan_contract, initiator, loan_id)?;
                PSP22BurnableRef::burn(&reserve_asset, borrow_amount)?;
                self._emit_repay_event(
                    loan_info.0,
                    collateral_asset,
                    borrow_asset,
                    collateral_amount,
                    to_repay,
                    0,
                );
            } else {
                PSP22Ref::transfer_from(&borrow_asset, initiator, contract, repay_amount, Vec::<u8>::new())?;
                let to_return = (repay_amount * collateral_amount) / to_repay;
                PSP22Ref::transfer(&collateral_asset, initiator, to_return, Vec::<u8>::new())?;
                PSP22MintableRef::mint(&reserve_asset, contract, to_repay - repay_amount - borrow_amount)?;
                LoanRef::update_loan(
                    &loan_contract,
                    loan_id,
                    to_repay - repay_amount,
                    Self::env().block_timestamp(),
                    collateral_amount - to_return,
                )?;
                self._emit_repay_event(
                    loan_info.0,
                    collateral_asset,
                    borrow_asset,
                    to_return,
                    repay_amount,
                    to_repay - repay_amount,
                );
            }
            Ok(true)
        }

        /// This function is called by the user who wants to withdraw assets they deposited for lending. They will deposit their
        /// share tokens and get back their share of the asset mapped to this share token
        ///
        /// `shares_address` account id of the shares token which is binded to the asset
        /// `shares_amount` amount of shares being withdrawn
        ///
        /// Returns `InsufficientBalanceInContract` if there is currently not enough assets in the contract
        #[ink(message)]
        pub fn withdraw_asset(
            &mut self,
            shares_address: AccountId,
            shares_amount: Balance,
        ) -> Result<(), LendingError> {
            let withdraw_asset = self._get_asset_from_shares(shares_address);
            let withdraw_amount =
                (shares_amount * self.total_asset(withdraw_asset)?) / PSP22Ref::total_supply(&shares_address);
            if withdraw_amount > PSP22Ref::balance_of(&withdraw_asset, Self::env().account_id()) {
                return Err(LendingError::InsufficientBalanceInContract)
            }
            PSP22BurnableRef::burn_from(&shares_address, Self::env().caller(), shares_amount)?;
            PSP22Ref::transfer(&withdraw_asset, Self::env().caller(), withdraw_amount, Vec::<u8>::new())?;
            self._emit_withdraw_event(Self::env().caller(), withdraw_asset, withdraw_amount);
            Ok(())
        }

        /// This function will liquidate the loan with `loan_id`. In this example contract the tokens will be kept in the smart
        /// contract and the liquidator gets 1% of the liquidated assets. In a real implementation we would swap the collateral
        /// for the borrowed asset so we would be able to cover the shares of lenders.
        ///
        /// `loan_id` id of loan to be liquidated
        ///
        /// Returns `LoanLiquidated` error if the loan was already liquidated
        /// Returns `CanNotBeLiquidated` error if the price of collateral is not below the liquidation price
        #[ink(message)]
        pub fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), LendingError> {
            let loan_contract = self.nft_contract;
            let loan_info = LoanRef::get_loan_info(&loan_contract, loan_id)?;
            let collateral_asset = loan_info.1;
            let collateral_amount = loan_info.2;
            let borrow_asset = loan_info.3;
            // let borrow_amount = loan_info.4;
            let liquidation_price = loan_info.5;
            let liquidated = loan_info.7;
            if liquidated {
                return Err(LendingError::LoanLiquidated)
            }
            let price = self._get_asset_price(collateral_amount, collateral_asset, borrow_asset);
            if price <= liquidation_price {
                // if we swapped the collateral to borrow asset we would burn the reserve tokens
                // let reserve_asset = self._get_reserve_asset(borrow_asset);
                // PSP22BurnableRef::burn(&reserve_asset, borrow_amount)
                let reward = (collateral_amount * 1000) / 100000;
                PSP22Ref::transfer(&collateral_asset, Self::env().caller(), reward, Vec::<u8>::new())?;
                LoanRef::liquidate_loan(&loan_contract, loan_id)?;
                self._emit_liquidate_event(
                    Self::env().caller(),
                    loan_info.0,
                    collateral_asset,
                    collateral_amount,
                    reward,
                );
            } else {
                return Err(LendingError::CanNotBeLiquidated)
            }
            Ok(())
        }

        /// This function will set price of `asset_in` in `asset_out` to `amount` in our simulated oracle
        #[modifiers(only_role(MANAGER))]
        #[ink(message)]
        pub fn set_asset_price(
            &mut self,
            asset_in: AccountId,
            asset_out: AccountId,
            price: Balance,
        ) -> Result<(), LendingError> {
            self._set_asset_price(asset_in, asset_out, price);
            Ok(())
        }

        // internal functions which can only be called inside our contract

        /// Internal function which will return the amount of `asset_out` we get
        /// when we deposit `amount_in` of `asset_in`
        /// This is enough for this example, but in a real application we would use an oracle for this
        fn _price_of(&self, amount_in: Balance, asset_in: AccountId, asset_out: AccountId) -> Balance {
            self._get_asset_price(amount_in, asset_in, asset_out)
        }

        /// internal function which instantiates a shares contract and returns its AccountId
        fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {
            let code_hash = self.code_hash;
            let contract = Shares::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
                .endowment(25)
                .code_hash(code_hash)
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate()
                .unwrap();
            contract.to_account_id()
        }

        /// internal function to emit an event when `manager_address` allows `asset_address` for lending
        fn _emit_lending_accepted_event(
            &self,
            asset_address: AccountId,
            shares_address: AccountId,
            reserves_address: AccountId,
            manager_address: AccountId,
        ) {
            self.env().emit_event(LendingAllowed {
                asset_address,
                shares_address,
                reserves_address,
                manager_address,
            });
        }

        /// internal function to emit an event when `manager_address` cancels lending of `asset_address`
        fn _emit_lending_cancelled_event(&self, asset_address: AccountId, manager_address: AccountId) {
            self.env().emit_event(LendingCancelled {
                asset_address,
                manager_address,
            });
        }

        /// internal function to emit an event when `manager_address` allows `asset_address` for collateral
        fn _emit_collateral_accepted_event(&self, asset_address: AccountId, manager_address: AccountId) {
            self.env().emit_event(CollateralAllowed {
                asset_address,
                manager_address,
            });
        }

        /// internal function to emit an event when `manager_address` cancels usage of `asset_address` as collateral
        fn _emit_collateral_cancelled_event(&self, asset_address: AccountId, manager_address: AccountId) {
            self.env().emit_event(CollateralCancelled {
                asset_address,
                manager_address,
            });
        }

        /// internal function to emit an event when `lender` deposits `amount` of token `asset`
        fn _emit_lend_event(&self, lender: AccountId, asset: AccountId, amount: Balance) {
            self.env().emit_event(Lend { lender, asset, amount });
        }

        /// internal function to emit an event when `borrower` borrows `borrow_amount` of `asset_address`
        /// while depositing `collateral_amount` of `collateral_address`
        fn _emit_borrow_event(
            &self,
            borrower: AccountId,
            collateral_address: AccountId,
            asset_address: AccountId,
            collateral_amount: Balance,
            borrow_amount: Balance,
        ) {
            self.env().emit_event(Borrow {
                borrower,
                collateral_address,
                asset_address,
                collateral_amount,
                borrow_amount,
            });
        }

        /// internal function to emit an event when `borrower` repays `repay_amount` of `asset_address`
        /// while getting back `collateral_amount` of `collateral_address`
        fn _emit_repay_event(
            &self,
            borrower: AccountId,
            collateral_address: AccountId,
            asset_address: AccountId,
            collateral_amount: Balance,
            repay_amount: Balance,
            to_repay: Balance,
        ) {
            self.env().emit_event(Repay {
                borrower,
                collateral_address,
                asset_address,
                collateral_amount,
                repay_amount,
                to_repay,
            });
        }

        /// internal function to emit an event when `lender` withdraws `withdraw_amount` of `asset_address`
        fn _emit_withdraw_event(&self, lender: AccountId, asset_address: AccountId, withdraw_amount: Balance) {
            self.env().emit_event(Withdraw {
                lender,
                asset_address,
                withdraw_amount,
            });
        }

        /// internal function to emit an event when `liquidator` liquidates a loan of `borrower`
        /// liquidating `collateral_amount` of `collateral_address` while getting `liquidator_fee`
        fn _emit_liquidate_event(
            &self,
            liquidator: AccountId,
            borrower: AccountId,
            collateral_address: AccountId,
            collateral_amount: Balance,
            liquidator_fee: Balance,
        ) {
            self.env().emit_event(Liquidate {
                liquidator,
                borrower,
                collateral_address,
                collateral_amount,
                liquidator_fee,
            });
        }
    }
}
