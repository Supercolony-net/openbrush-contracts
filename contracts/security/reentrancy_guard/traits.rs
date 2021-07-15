pub use brush::{modifiers, modifier_definition};
pub use ink_lang::{Env, StaticEnv};
pub use reentrancy_guard_derive::ReentrancyGuardStorage;
use brush::traits::Flush;

// We don't need to expose it, because ink! will define StaticEnv itself.
use brush::traits::{InkStorage};

const NOT_ENTERED: u8 = 0;
const ENTERED: u8 = 1;

#[brush::storage_trait]
pub trait ReentrancyGuardStorage: InkStorage {
    fn _status(&self) -> & u8;
    fn _status_mut(&mut self) -> &mut u8;
}

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
    assert_eq!(instance._status(), &NOT_ENTERED, "{}", ReentrancyGuardError::ReentrantCall.as_ref());
    // Any calls to nonReentrant after this point will fail
    *instance._status_mut() = ENTERED;

    // We want to flush storage before execution of inner function,
    // because ink! doesn't do it by default and `status` will be not updated in child calls
    instance.flush();

    let result = body(instance);
    *instance._status_mut() = NOT_ENTERED;

    return result;
}
