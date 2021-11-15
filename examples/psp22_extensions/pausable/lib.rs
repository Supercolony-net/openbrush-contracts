#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_pausable {
    use pausable::traits::*;
    use psp22::traits::*;

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PausableStorage)]
    pub struct MyPSP22Pausable {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PausableStorageField]
        pause: PausableData,
    }

    impl PSP22 for MyPSP22Pausable {
        /// Return `Paused` error if the token is paused
        fn _before_token_transfer(
            &mut self,
            _from: &AccountId,
            _to: &AccountId,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            // TODO logic for before token transfer
            if self.paused() {
                return Err(PSP22Error::from(PausableError::Paused))
            }
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
