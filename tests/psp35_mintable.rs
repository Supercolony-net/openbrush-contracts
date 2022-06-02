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
#[cfg(feature = "psp35")]
#[openbrush::contract]
mod psp35_mintable {
    use ink_lang as ink;
    use openbrush::test_utils::accounts;
    use openbrush_contracts::psp35::extensions::mintable::*;

    #[derive(Default, PSP35Storage)]
    #[ink(storage)]
    pub struct PSP35Struct {
        #[PSP35StorageField]
        psp35: PSP35Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
    }

    impl PSP35Mintable for PSP35Struct {}
    impl PSP35 for PSP35Struct {}

    impl PSP35Transfer for PSP35Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP35Error> {
            if self.return_err_on_before {
                return Err(PSP35Error::Custom(String::from("Error on _before_token_transfer")))
            }
            Ok(())
        }
    }

    impl PSP35Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }
    }

    #[ink::test]
    fn mint_works() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();

        let mut nft = PSP35Struct::new();
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), 0);

        assert!(nft.mint(accounts.alice, vec![(token_id_1, token_1_amount)]).is_ok());
        assert!(nft.mint(accounts.bob, vec![(token_id_2, token_2_amount)]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), token_2_amount);
    }

    #[ink::test]
    fn before_token_transfer_should_fail_mint() {
        let token_id = [1; 32];
        let amount = 1;
        let accounts = accounts();
        let mut nft = PSP35Struct::new();
        // Can mint
        assert!(nft.mint(accounts.alice, vec![(token_id, amount)]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id), amount);
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, vec![(token_id, amount)]),
            Err(PSP35Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }
}
