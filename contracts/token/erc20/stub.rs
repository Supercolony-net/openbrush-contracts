use ink_lang as ink;
pub use self::erc20::{Erc20};

#[ink::contract]
mod erc20 {
    use ink_prelude::string::String;
    use crate::traits::Erc20Error;

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc20 {}

    #[ink(namespace = "IErc20")]
    impl Erc20 {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_name(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_symbol(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_decimals(&self) -> Option<u8> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<(), Erc20Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<(), Erc20Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<(), Erc20Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn increase_allowance(&mut self, _spender: AccountId, _delta_value: Balance) -> Result<(), Erc20Error> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn decrease_allowance(&mut self, _spender: AccountId, _delta_value: Balance) -> Result<(), Erc20Error> {
            unimplemented!()
        }
    }
}