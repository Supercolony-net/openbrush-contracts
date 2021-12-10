---
sidebar_position: 7
title: Errors
---

We will define errors thrown by our contract in a separate file `errors.rs`. In this file we will define the errors that will be returned by our contract, and implement conversion of OpenBrush errors (`AccessControlError`, `PSP22Error` and `PSP721Error`) to our error.

## Define errors

```rust
use access_control::traits::AccessControlError;
use ink_prelude::string::String;
use pausable::traits::PausableError;
use psp22::traits::PSP22Error;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LendingError {
    Custom(String),
    InsufficientAllowanceToLend,
    InsufficientBalanceToLend,
    InsufficientAllowanceToRepay,
    InsufficientBalanceToRepay,
    InsufficientAllowanceForCollateral,
    InsufficientCollateralBalance,
    AmountNotSupported,
    InsufficientBalanceInContract,
    AssetNotSupported,
    AssetSupported,
    NotTheOwner,
    LoanLiquidated,
    CanNotBeLiquidated,
    AssetsInTheContract,
}
```

## Implement conversion from OpenBrush errors

```rust
impl From<AccessControlError> for LendingError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => LendingError::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => LendingError::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => LendingError::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for LendingError {
    fn from(access: PausableError) -> Self {
        match access {
            PausableError::Paused => LendingError::Custom(String::from("Paused")),
            PausableError::NotPaused => LendingError::Custom(String::from("Not Paused")),
        }
    }
}

impl From<PSP22Error> for LendingError {
    fn from(error: PSP22Error) -> Self {
        match error {
            PSP22Error::Custom(message) => LendingError::Custom(message),
            PSP22Error::InsufficientBalance => LendingError::Custom(String::from("PSP22::InsufficientBalance")),
            PSP22Error::InsufficientAllowance => LendingError::Custom(String::from("PSP22::InsufficientAllowance")),
            PSP22Error::ZeroRecipientAddress => LendingError::Custom(String::from("PSP22::ZeroRecipientAddress")),
            PSP22Error::ZeroSenderAddress => LendingError::Custom(String::from("PSP22::ZeroSenderAddress")),
            PSP22Error::SafeTransferCheckFailed(message) => LendingError::Custom(message),
        }
    }
}

impl From<PSP721Error> for LendingError {
    fn from(error: PSP721Error) -> Self {
        match error {
            PSP721Error::Custom(message) => LendingError::Custom(message),
            PSP721Error::SelfApprove => LendingError::Custom(String::from("PSP721::SelfApprove")),
            PSP721Error::NotApproved => LendingError::Custom(String::from("PSP721::NotApproved")),
            PSP721Error::TokenExists => LendingError::Custom(String::from("PSP721::TokenExists")),
            PSP721Error::TokenNotExists => LendingError::Custom(String::from("PSP721::TokenNotExists")),
            PSP721Error::SafeTransferCheckFailed(message) => LendingError::Custom(message),
        }
    }
}
```