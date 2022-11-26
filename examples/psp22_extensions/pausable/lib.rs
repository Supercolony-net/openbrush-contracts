#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pausable {
    use openbrush::{
        contracts::{
            pausable::*,
            psp22::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        pause: pausable::Data,
    }

    impl PSP22 for Contract {}

    impl Transfer for Contract {
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

    impl Pausable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());
            })
        }

        /// Function which changes state to unpaused if paused and vice versa
        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PSP22Error> {
            self._switch_pause()
        }
    }
}
