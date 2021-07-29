use brush::modifier_definition;
use brush::traits::{InkStorage, Flush};
use brush::declare_storage_trait;
use ink_storage::{
    traits::{SpreadLayout},
};
pub use reentrancy_guard_derive::ReentrancyGuardStorage;

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

#[derive(strum_macros::AsRefStr)]
pub enum ReentrancyGuardError {
    ReentrantCall,
}

#[modifier_definition]
pub fn non_reentrant<T, F, ReturnType>(instance: &mut T, mut body: F) -> ReturnType
    where
        T: ReentrancyGuardStorage + Flush,
        F: FnMut(&mut T) -> ReturnType,
{
    assert_eq!(instance.get().status, NOT_ENTERED, "{}", ReentrancyGuardError::ReentrantCall.as_ref());
    // Any calls to nonReentrant after this point will fail
    instance.get_mut().status = ENTERED;

    // We want to flush storage before execution of inner function,
    // because ink! doesn't do it by default and `status` will not be updated in child calls
    instance.flush();

    let result = body(instance);
    instance.get_mut().status = NOT_ENTERED;

    return result;
}
