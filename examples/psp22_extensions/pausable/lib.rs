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
            let mut instance = Self::default();

            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());

            instance
        }

        /// Function which changes state to unpaused if paused and vice versa
        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PSP22Error> {
            self._switch_pause()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22::psp22_external::PSP22;
        use openbrush::contracts::pausable::pausable_external::Pausable;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn can_transfer_when_not_paused(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000);
            let address = client.instantiate("my_psp22_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));
            assert!(matches!(balance_of!(client, address, bob), 0));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 100, vec![]));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }.return_value();

            assert!(matches!(transfer_tx, Ok(())));

            assert!(matches!(balance_of!(client, address, alice), 900));
            assert!(matches!(balance_of!(client, address, bob), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_when_paused(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000);
            let address = client.instantiate("my_psp22_pausable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));
            assert!(matches!(balance_of!(client, address, bob), 0));

            let pause_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.change_state());
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("change_state failed")
            }.return_value();

            assert!(matches!(pause_tx, Ok(())));

            let transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 100, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(transfer_tx, Err(_)));

            assert!(matches!(balance_of!(client, address, alice), 1000));
            assert!(matches!(balance_of!(client, address, bob), 0));

            Ok(())
        }
    }
}
