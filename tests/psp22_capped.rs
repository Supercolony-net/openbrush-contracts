// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(min_specialization)]
#[cfg(feature = "psp22")]
#[openbrush::contract]
mod psp22_capped {
    use ink::storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::extensions::{
            capped::*,
            mintable::*,
        },
        test_utils::accounts,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22Struct {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        cap: Data,
    }

    impl PSP22 for PSP22Struct {}

    impl PSP22Capped for PSP22Struct {}

    impl PSP22Mintable for PSP22Struct {}

    impl psp22::Transfer for PSP22Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            // `is_none` means that it is minting
            if _from.is_none() && self._is_cap_exceeded(_amount) {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")))
            }
            Ok(())
        }
    }

    impl PSP22Struct {
        /// Constructor which mints `initial_supply` of the token to sender
        /// Will set the token's cap to `cap`
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._init_cap(CAP).is_ok());
                assert!(instance.mint(instance.env().caller(), 1).is_ok());
            })
        }
    }

    const CAP: u128 = 1000;

    #[ink::test]
    fn initializing_works() {
        let token = PSP22Struct::new();
        assert_eq!(token.cap(), CAP);
    }

    #[ink::test]
    fn mint_works() {
        let mut token = PSP22Struct::new();

        let accounts = accounts();
        let alice_balance = token.balance_of(accounts.alice);
        assert!(token.mint(accounts.alice, 1).is_ok());
        assert_eq!(token.balance_of(accounts.alice), alice_balance + 1);
    }

    #[ink::test]
    fn mint_fails() {
        let mut token = PSP22Struct::new();

        let accounts = accounts();
        let alice_balance = token.balance_of(accounts.alice);
        assert_eq!(
            token.mint(accounts.alice, CAP),
            Err(PSP22Error::Custom(String::from("Cap exceeded")))
        );
        assert_eq!(token.balance_of(accounts.alice), alice_balance);
    }
}
