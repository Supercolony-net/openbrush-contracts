use super::{
    AccessControlError,
    OwnableError,
    PausableError,
    ReentrancyGuardError,
};
use ink_prelude::string::String;

/// The TimelockController error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TimelockControllerError {
    Custom(String),
    AccessControlError(AccessControlError),
    InsufficientDelay,
    OperationAlreadyScheduled,
    OperationCannonBeCanceled,
    OperationIsNotReady,
    MissingDependency,
    UnderlyingTransactionReverted,
    CallerMustBeTimeLock,
}

impl From<AccessControlError> for TimelockControllerError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                TimelockControllerError::AccessControlError(AccessControlError::MissingRole)
            }
            AccessControlError::RoleRedundant => {
                TimelockControllerError::AccessControlError(AccessControlError::RoleRedundant)
            }
            AccessControlError::InvalidCaller => {
                TimelockControllerError::AccessControlError(AccessControlError::InvalidCaller)
            }
        }
    }
}

impl From<OwnableError> for TimelockControllerError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => TimelockControllerError::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => TimelockControllerError::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<PausableError> for TimelockControllerError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => TimelockControllerError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => TimelockControllerError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for TimelockControllerError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => TimelockControllerError::Custom(String::from("RG::ReentrantCall")),
        }
    }
}
