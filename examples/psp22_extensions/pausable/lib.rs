#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_pausable {
    use brush::{
        contracts::{
            pausable::*,
            psp22::*,
        },
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PausableStorage)]
    pub struct MyPSP22Pausable {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PausableStorageField]
        pause: PausableData,
    }

    impl PSP22 for MyPSP22Pausable {}

    impl PSP22Internal for MyPSP22Pausable {
        /// Return `Paused` error if the token is paused
        #[modifiers(when_not_paused)]
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            // TODO logic for before token transfer
            Ok(())
        }
    }

    impl Pausable for MyPSP22Pausable {}

    impl MyPSP22Pausable {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(Self::env().caller(), total_supply).is_ok());
            instance
        }

        /// Function which changes state to unpaused if paused and vice versa
        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PSP22Error> {
            if self.paused() {
                self._unpause()
            } else {
                self._pause()
            }
        }
    }
}
