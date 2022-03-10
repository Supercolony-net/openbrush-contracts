pub use crate::traits::errors::ReentrancyGuardError;
use brush::{
    declare_storage_trait,
    modifier_definition,
};
pub use derive::ReentrancyGuardStorage;
use ink_storage::traits::push_spread_root;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::ReentrancyGuardData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct ReentrancyGuardData {
    pub status: u8,
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
    push_spread_root(instance.get(), &Default::default());

    let result = body(instance);
    instance.get_mut().status = NOT_ENTERED;

    return result
}
