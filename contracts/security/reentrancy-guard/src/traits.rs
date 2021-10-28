use brush::{
    declare_storage_trait,
    modifier_definition,
    traits::{
        Flush,
        InkStorage,
    },
};
use ink_storage::traits::SpreadLayout;
pub use reentrancy_guard_derive::ReentrancyGuardStorage;
use scale::{
    Decode,
    Encode,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ReentrancyGuardData {
    pub status: u8,
}

declare_storage_trait!(ReentrancyGuardStorage, ReentrancyGuardData);

const NOT_ENTERED: u8 = 0;
const ENTERED: u8 = 1;

#[derive(strum_macros::AsRefStr, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ReentrancyGuardError {
    ReentrantCall,
}

/// Prevents a contract from calling itself, directly or indirectly.
/// Calling a `non_reentrant` function from another `non_reentrant`
/// function is not supported. It is possible to prevent this from happening
/// by making the `non_reentrant` function external, and make it call a
/// `private` function that does the actual work.
///
/// This modifier flushes the struct into storage with `ENTERED`
/// status before calling the original method.
#[modifier_definition]
pub fn non_reentrant<T, F, ReturnType>(instance: &mut T, body: F) -> ReturnType
where
    T: ReentrancyGuardStorage + Flush,
    F: FnOnce(&mut T) -> ReturnType,
{
    assert_eq!(
        instance.get().status,
        NOT_ENTERED,
        "{}",
        ReentrancyGuardError::ReentrantCall.as_ref()
    );
    // Any calls to nonReentrant after this point will fail
    instance.get_mut().status = ENTERED;

    // We want to flush storage before execution of inner function,
    // because ink! doesn't do it by default and `status` will not be updated in child calls
    instance.flush();

    let result = body(instance);
    instance.get_mut().status = NOT_ENTERED;

    return result
}
