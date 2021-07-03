pub use brush::{modifiers, modifier_definition};
pub use ink_lang::{Env, StaticEnv};
pub use reentrancy_guard_derive::ReentrancyGuardStorage;
pub use brush::traits::Flush;

// We don't need to expose it, because ink! will define StaticEnv by self.
use brush::traits::{InkStorage};

pub const NOT_ENTERED: u8 = 0;
pub const ENTERED: u8 = 1;

#[brush::storage_trait]
pub trait ReentrancyGuardStorage: InkStorage {
    fn _status(&self) -> & u8;
    fn _status_mut(&mut self) -> &mut u8;
}

#[derive(strum_macros::AsRefStr)]
pub enum ReentrancyGuardError {
    ReentrantCall,
}

pub trait ReentrancyGuardModifier: ReentrancyGuardStorage + Flush {
    #[modifier_definition]
    fn non_reentrant(&mut self) {
        assert_eq!(self._status(), &NOT_ENTERED, "{}", ReentrancyGuardError::ReentrantCall.as_ref());
        // Any calls to nonReentrant after this point will fail
        *self._status_mut() = ENTERED;

        // We want to flush storage before execution of inner function.
        // Because ink! doesn't do it by default and `status` will be not updated in child calls
        self.flush();

        #[body]();
        *self._status_mut() = NOT_ENTERED;
    }
}
