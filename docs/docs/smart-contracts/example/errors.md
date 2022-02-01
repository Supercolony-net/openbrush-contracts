---
sidebar_position: 8
title: Errors
---

We will define errors thrown by the lending contract at end of `traits/lending.rs`
because only that contract returns its own errors. But if you have more than one error definition,
better to create a separate `traits/errors.rs` file for them(or a directory `traits/errors/`).
In that file(directory) you can define the errors that will be returned by your contracts,
and implement conversion between different errors.
In the project, we implement the conversion for some errors from OpenBrush.

## Define errors

```rust
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
```

## Implement conversion from OpenBrush errors

```rust
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
```