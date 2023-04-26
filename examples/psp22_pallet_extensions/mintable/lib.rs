#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![feature(default_alloc_error_handler)]

#[openbrush::contract]
pub mod my_psp22_pallet_mintable {
    use openbrush::{
        contracts::psp22_pallet::extensions::mintable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {
            let mut instance = Self::default();

            instance
                ._create(asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id = asset_id;
            instance.pallet.origin = Origin::Caller;
            instance
                ._mint_to(instance.env().caller(), total_supply)
                .expect("Should mint_to");

            instance
        }

        #[ink(message)]
        pub fn mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self.mint(account, amount)
        }

    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22_pallet::psp22_external::PSP22;
        use openbrush::contracts::psp22_pallet::extensions::mintable::psp22mintable_external::PSP22Mintable;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of
        };

        fn random_num() -> u32 {
            use rand::Rng;
            rand::thread_rng().gen_range(1..1000)
        }

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let address = client.instantiate("my_psp22_pallet_mintable", &ink_e2e::alice(), constructor, 1000000000000000000, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn minting_requested_amount(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 1000);
            let address = client.instantiate("my_psp22_pallet_mintable", &ink_e2e::alice(), constructor, 1000000000000000000, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, bob), 0), "Bob's balance should be 0");

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint_to(address_of!(bob), 1000));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(mint_tx.return_value(), Ok(())), "Minting should be successful");

            assert!(matches!(balance_of!(client, address, bob), 1000), "Bob's balance should be 1000");

            Ok(())
        }

        #[ink_e2e::test]
        async fn increases_total_supply_after_minting(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(random_num(), 1, 0);
            let address = client.instantiate("my_psp22_pallet_mintable", &ink_e2e::alice(), constructor, 1000000000000000000, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(total_supply, 0), "Total supply should be 0");

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint_to(address_of!(bob), 1000));
                client.call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(mint_tx.return_value(), Ok(())), "Minting should be successful");

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert!(matches!(total_supply, 1000), "Total supply should be 1000");

            Ok(())
        }
    }
}
