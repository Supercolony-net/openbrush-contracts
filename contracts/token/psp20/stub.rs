// It exports the stub implementation of all PSP20 traits.
// ink! will generate a wrapper around all methods of each trait and it will allow creating wrapped
// struct around contracts address(::ink_env::call::FromAccountId::from_account_id).
#[cfg(not(test))]
pub use self::psp20::{PSP20};

#[cfg(not(test))]
#[ink_lang::contract]
mod psp20 {
    use ink_prelude::string::String;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP20 {}

    #[ink(namespace = "IPSP20")]
    impl PSP20 {
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
        pub fn transfer(&mut self, _to: AccountId, _value: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, _from: AccountId, _to: AccountId, _value: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn increase_allowance(&mut self, _spender: AccountId, _delta_value: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn decrease_allowance(&mut self, _spender: AccountId, _delta_value: Balance) {
            unimplemented!()
        }
    }
}