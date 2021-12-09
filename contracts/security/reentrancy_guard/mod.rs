pub use crate::traits::errors::ReentrancyGuardError;
use brush::{
    declare_storage_trait,
    modifier_definition,
};
pub use derive::ReentrancyGuardStorage;
use ink_primitives::{
    Key,
    KeyPtr,
};
use ink_storage::traits::{
    push_spread_root,
    SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ReentrancyGuardData {
    pub status: u8,
}

pub const REENTRANCY_GUARD_DATA_STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("ReentrancyGuardData");

/// ReentrancyGuardData has own storage key
impl SpreadLayout for ReentrancyGuardData {
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = false;

    fn pull_spread(_: &mut KeyPtr) -> Self {
        let mut ptr = KeyPtr::from(Key::from(REENTRANCY_GUARD_DATA_STORAGE_KEY));
        Self {
            status: SpreadLayout::pull_spread(&mut ptr),
        }
    }

    fn push_spread(&self, _: &mut KeyPtr) {
        let mut ptr = KeyPtr::from(Key::from(REENTRANCY_GUARD_DATA_STORAGE_KEY));
        SpreadLayout::push_spread(&self.status, &mut ptr);
    }

    fn clear_spread(&self, _: &mut KeyPtr) {
        let mut ptr = KeyPtr::from(Key::from(REENTRANCY_GUARD_DATA_STORAGE_KEY));
        SpreadLayout::clear_spread(&self.status, &mut ptr);
    }
}

declare_storage_trait!(ReentrancyGuardStorage, ReentrancyGuardData);

const NOT_ENTERED: u8 = 0;
const ENTERED: u8 = 1;

/// Prevents a contract from calling itself, directly or indirectly.
/// Calling a `non_reentrant` function from another `non_reentrant`
/// function is not supported. It is possible to prevent this from happening
/// by making the `non_reentrant` function external, and make it call a
/// `private` function that does the actual work.
///
/// This modifier flushes the struct into storage with `ENTERED`
/// status before calling the original method.
#[modifier_definition]
pub fn non_reentrant<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: ReentrancyGuardStorage,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<ReentrancyGuardError>,
{
    if instance.get().status == ENTERED {
        return Err(From::from(ReentrancyGuardError::ReentrantCall))
    }
    // Any calls to nonReentrant after this point will fail
    instance.get_mut().status = ENTERED;

    // We want to flush storage before execution of inner function,
    // because ink! doesn't do it by default and `status` will not be updated in child calls
    push_spread_root(instance.get(), &Key::default());

    let result = body(instance);
    instance.get_mut().status = NOT_ENTERED;

    return result
}
