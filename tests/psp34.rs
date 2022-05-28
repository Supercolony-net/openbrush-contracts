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
#[cfg(feature = "psp34")]
#[openbrush::contract]
mod psp34 {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_env::DefaultEnvironment;
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::*,
        test_utils::{
            accounts,
            change_caller,
        },
    };

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    #[derive(Default, SpreadAllocate, PSP34Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP34Internal for PSP34Struct {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
            self.env().emit_event(Approval { from, to, id, approved });
        }

        fn _do_safe_transfer_check(
            &mut self,
            _operator: &AccountId,
            _from: &AccountId,
            _to: &AccountId,
            _id: &Id,
            _data: &Vec<u8>,
        ) -> Result<(), PSP34Error> {
            Ok(())
        }
    }

    impl PSP34Transfer for PSP34Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_before {
                return Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_after {
                return Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
            }
            Ok(())
        }
    }

    impl PSP34 for PSP34Struct {}

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut PSP34Struct| {})
        }

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
        }
    }

    #[ink::test]
    fn collection_id_works() {
        assert_eq!(
            PSP34Struct::new().collection_id(),
            Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&ink_env::account_id::<DefaultEnvironment>()).to_vec())
        );
    }

    #[ink::test]
    fn transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns token 1
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns any token
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // The first Transfer event takes place
        assert_eq!(1, ink_env::test::recorded_events().count());
        // Alice transfers token 1 to Bob
        assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // The second Transfer event takes place
        assert_eq!(2, ink_env::test::recorded_events().count());
        // Bob owns token 1
        assert_eq!(nft.balance_of(accounts.bob), 1);
        // Alice doesn't own token 1
        assert_eq!(nft.balance_of(accounts.alice), 0);
    }

    #[ink::test]
    fn not_exist_token_transfer_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Transfer token fails if it does not exists.
        assert_eq!(
            nft.transfer(accounts.bob, Id::U8(1u8), vec![]),
            Err(PSP34Error::TokenNotExists)
        );
        // Token Id 2 does not exists.
        assert_eq!(nft.owner_of(Id::U8(1u8)), None);
    }

    #[ink::test]
    fn not_owned_token_transfer_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 2.
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Token Id 2 is owned by Alice.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        change_caller(accounts.bob);
        // Bob cannot transfer not owned tokens.
        assert_eq!(
            nft.transfer(accounts.eve, Id::U8(1u8), vec![]),
            Err(PSP34Error::NotApproved)
        );
    }

    #[ink::test]
    fn approve_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());

        // Token 1 is not approved
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, Some(Id::U8(1u8))), false);

        assert!(nft.approve(accounts.bob, Some(Id::U8(1u8)), true).is_ok());
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, Some(Id::U8(1u8))), true);
    }

    #[ink::test]
    fn approve_works_fails() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert_eq!(
            nft.approve(accounts.bob, Some(Id::U8(1u8)), true),
            Err(PSP34Error::TokenNotExists)
        );
        assert_eq!(
            nft.approve(accounts.bob, Some(Id::U8(1u8)), false),
            Err(PSP34Error::TokenNotExists)
        );

        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert_eq!(
            nft.approve(accounts.alice, Some(Id::U8(1u8)), true),
            Err(PSP34Error::SelfApprove)
        );

        change_caller(accounts.bob);
        assert_eq!(
            nft.approve(accounts.eve, Some(Id::U8(1u8)), true),
            Err(PSP34Error::NotApproved)
        );
        assert_eq!(
            nft.approve(accounts.eve, Some(Id::U8(1u8)), false),
            Err(PSP34Error::NotApproved)
        );
    }

    #[ink::test]
    fn approved_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Token Id 1 is owned by Alice.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert!(nft.approve(accounts.bob, Some(Id::U8(1u8)), true).is_ok());
        // Get contract address.
        change_caller(accounts.bob);
        // Bob transfers token Id 1 from Alice to Eve.
        assert!(nft.transfer(accounts.eve, Id::U8(1u8), vec![]).is_ok());
        // TokenId 3 is owned by Eve.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.eve));
        // Alice has one token left
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 1 token.
        assert_eq!(nft.balance_of(accounts.eve), 1);
    }

    #[ink::test]
    fn total_supply_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert_eq!(nft.total_supply(), 0);
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // 1 tokens minted in total
        assert_eq!(nft.total_supply(), 1);
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // 2 tokens minted in total
        assert_eq!(nft.total_supply(), 2);
        assert!(nft._mint_to(accounts.alice, Id::U8(3u8)).is_ok());
        // 3 tokens minted in total
        assert_eq!(nft.total_supply(), 3)
    }

    #[ink::test]
    fn approved_for_all_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert!(nft.approve(accounts.bob, None, true).is_ok());
        // Bob is an approved operator for Alice
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, None), true);

        change_caller(accounts.bob);
        // Bob transfers token Id 1 from Alice to Eve.
        assert!(nft.transfer(accounts.eve, Id::U8(1u8), vec![]).is_ok());
        // TokenId 1 is owned by Eve.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.eve));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob transfers token Id 2 from Alice to Eve.
        assert!(nft.transfer(accounts.eve, Id::U8(2u8), vec![]).is_ok());
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.eve), 2);

        change_caller(accounts.alice);
        // Remove operator approval for Bob on behalf of Alice.
        assert!(nft.approve(accounts.bob, None, false).is_ok());
        // Bob is not an approved operator for Alice.
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, None), false);
    }

    #[ink::test]
    fn operator_approve_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns token.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Approve token Bob on behalf of Alice.
        assert!(nft.approve(accounts.bob, None, true).is_ok());
        // Bob is an approved operator for Alice
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, None), true);

        // Eve is not approved to send token 1 from Alice
        assert_eq!(nft.allowance(accounts.alice, accounts.eve, Some(Id::U8(1u8))), false);
        change_caller(accounts.bob);
        // Approve token Id 1 Eve by operator Bob.
        assert!(nft.approve(accounts.eve, Some(Id::U8(1u8)), true).is_ok());
        // Eve is approved to send token 1 from Alice by operator
        assert_eq!(nft.allowance(accounts.alice, accounts.eve, Some(Id::U8(1u8))), true);
    }

    #[ink::test]
    fn not_approved_transfer_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve does not owns tokens.
        assert_eq!(nft.balance_of(accounts.eve), 0);
        // Get contract address.
        change_caller(accounts.bob);
        // Eve is not an approved operator by Alice.
        assert_eq!(
            nft.transfer(accounts.frank, Id::U8(1u8), vec![]),
            Err(PSP34Error::NotApproved)
        );
    }

    #[ink::test]
    fn before_token_transfer_should_fail_transfer() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(4u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can transfer token
        assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.transfer(accounts.bob, Id::U8(4u8), vec![]),
            Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_transfer() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(4u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can transfer token
        assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.transfer(accounts.bob, Id::U8(4u8), vec![]),
            Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
