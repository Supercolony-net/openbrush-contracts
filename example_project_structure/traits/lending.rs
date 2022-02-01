use brush::{
    contracts::traits::{
        access_control::*,
        pausable::*,
        psp22::PSP22Error,
        psp34::{
            Id,
            PSP34Error,
        },
    },
    traits::{
        AccountId,
        Balance,
    },
};

/// Combination of all traits of the contract to simplify calls to the contract
#[brush::wrapper]
pub type LendingContractRef = dyn Lending + LendingPermissioned + AccessControl + Pausable;

#[brush::wrapper]
pub type LendingRef = dyn Lending;

#[brush::trait_definition]
pub trait Lending {
    /// This function will return the total amount of assets available to borrow
    /// along with amount of the same asset borrowed
    ///
    /// Returns `AssetNotSupported` error if we try to get amount of asset not supported by our contract
    #[ink(message)]
    fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError>;

    /// This function will return the total amount of shares minted for an asset
    ///
    /// Returns `AssetNotSupported` error if we try to get shares of asset not supported by our contract
    #[ink(message)]
    fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError>;

    /// This function will return true if the asset is accepted by the contract
    #[ink(message)]
    fn is_accepted_lending(&self, asset_address: AccountId) -> bool;

    /// This function will return true if the asset is accepted by the contract
    #[ink(message)]
    fn is_accepted_collateral(&self, asset_address: AccountId) -> bool;

    /// This function is called by a user who wants to lend tokens and gain interest
    ///
    /// `asset_address` is the AccountId of the PSP-22 token to be deposited
    /// `amount` is the amount to be deposited
    ///
    /// Returns `InsufficientAllowanceToLend` if the caller does not have enough allowance
    /// Returns `InsufficientBalanceToLend` if the caller does not have enough balance
    /// Returns `AssetNotSupported` if the asset is not supported for lending
    #[ink(message)]
    fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError>;

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
    #[ink(message)]
    fn borrow_assets(
        &mut self,
        asset_address: AccountId,
        collateral_address: AccountId,
        amount: Balance,
    ) -> Result<(), LendingError>;

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
    fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, LendingError>;

    /// This function is called by the user who wants to withdraw assets they deposited for lending. They will deposit their
    /// share tokens and get back their share of the asset mapped to this share token
    ///
    /// `shares_address` account id of the shares token which is binded to the asset
    /// `shares_amount` amount of shares being withdrawn
    ///
    /// Returns `InsufficientBalanceInContract` if there is currently not enough assets in the contract
    #[ink(message)]
    fn withdraw_asset(&mut self, shares_address: AccountId, shares_amount: Balance) -> Result<(), LendingError>;

    /// This function will liquidate the loan with `loan_id`. In this example contract the tokens will be kept in the smart
    /// contract and the liquidator gets 1% of the liquidated assets. In a real implementation we would swap the collateral
    /// for the borrowed asset so we would be able to cover the shares of lenders.
    ///
    /// `loan_id` id of loan to be liquidated
    ///
    /// Returns `LoanLiquidated` error if the loan was already liquidated
    /// Returns `CanNotBeLiquidated` error if the price of collateral is not below the liquidation price
    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), LendingError>;
}

#[brush::wrapper]
pub type LendingPermissionedRef = dyn LendingPermissioned;

#[brush::trait_definition]
pub trait LendingPermissioned {
    /// This function will allow an asset to be accepted by the contract
    /// It will also create the contracts for the shares token and lended reserves token
    #[ink(message, payable)]
    fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

    /// This function will disallow lending and borrowing of asset
    /// To do this all assets of this asset must be repaid and all of the asset must be withdrawn
    #[ink(message)]
    fn disallow_lending(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

    /// This function will allow an asset to be accepted as collateral
    #[ink(message)]
    fn allow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

    /// This function will disallow an asset to be accepted as collateral
    #[ink(message)]
    fn disallow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

    /// This function will set price of `asset_in` in `asset_out` to `amount` in our simulated oracle
    #[ink(message)]
    fn set_asset_price(
        &mut self,
        asset_in: AccountId,
        asset_out: AccountId,
        price: Balance,
    ) -> Result<(), LendingError>;
}

/// Enum of errors raised by our lending smart contract
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LendingError {
    PSP22Error(PSP22Error),
    PSP34Error(PSP34Error),
    AccessControlError(AccessControlError),
    PausableError(PausableError),

    /// This error will be thrown when the lender does not have enough allowance
    /// to transfer the lending asset to the contract
    InsufficientAllowanceToLend,
    /// This error will be thrown when the lender tries to lend more amount of asset than they own
    InsufficientBalanceToLend,
    /// This error will be thrown when the borrower does not have enough allowance
    /// to transfer the borrowed asset to the contract
    InsufficientAllowanceToRepay,
    /// This error will be thrown when the borrower tries to repay more amount of asset than they own
    InsufficientBalanceToRepay,
    /// This error will be thrown when the borrower does not have enough allowance
    /// to transfer the collateral asset to the contract
    InsufficientAllowanceForCollateral,
    /// This error will be thrown when the borrower tries to use more amount of asset as collateral than they own
    InsufficientCollateralBalance,
    // This error will be thrown if the amount of borrowed assets is greater than or equal to the liquidation price of deposited collateral
    AmountNotSupported,
    // This error will be thrown if the user wants to borrow or withdraw more assets than there currently are in the contract
    InsufficientBalanceInContract,
    /// This error will be thrown if the user tries to lend or borrow asset which is not supported by the lending contract
    /// or if a user tries to use an usupported asset as a collateral
    AssetNotSupported,
    /// This error will be thrown if the user tries to allow an asset which is already allowed
    AssetSupported,
    /// This error will be thrown if the user tries to repay a loan he does not own
    NotTheOwner,
    /// This error will be thrown if the loan we try to liquidate was already liquidated
    LoanLiquidated,
    /// This error will be thrown if the loan we try to liquidate is not below liquidation price
    CanNotBeLiquidated,
    /// This error will be thrown if an user wants to disallow lending of an asset which is still present in the contract
    AssetsInTheContract,
}

impl From<AccessControlError> for LendingError {
    fn from(access: AccessControlError) -> Self {
        LendingError::AccessControlError(access)
    }
}

impl From<PausableError> for LendingError {
    fn from(access: PausableError) -> Self {
        LendingError::PausableError(access)
    }
}

impl From<PSP22Error> for LendingError {
    fn from(error: PSP22Error) -> Self {
        LendingError::PSP22Error(error)
    }
}

impl From<PSP34Error> for LendingError {
    fn from(error: PSP34Error) -> Self {
        LendingError::PSP34Error(error)
    }
}
