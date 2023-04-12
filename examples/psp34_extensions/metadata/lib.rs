#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Metadata for Contract {}

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            instance._set_attribute(id.clone(), name_key, name);
            instance._set_attribute(id, symbol_key, symbol);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp34::extensions::metadata::psp34metadata_external::PSP34Metadata;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use openbrush::traits::String;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let id = Id::U8(0);
            let name = String::from("My PSP34");
            let symbol = String::from("MPS34");

            let constructor = ContractRef::new(id.clone(), name.clone(), symbol.clone());
            let address = client.instantiate("my_psp34_metadata", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result_name = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), String::from("name")));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            let result_symbol = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), String::from("symbol")));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
            }.return_value();

            assert_eq!(result_name, Some(name));
            assert_eq!(result_symbol, Some(symbol));

            Ok(())
        }
    }
}
