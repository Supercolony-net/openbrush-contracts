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
#[brush::contract]
mod psp35 {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp35::*;
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        id: Option<Id>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        value: Balance,
    }

    #[derive(Default, SpreadAllocate, PSP35Storage)]
    #[ink(storage)]
    pub struct PSP35Struct {
        #[PSP35StorageField]
        psp35: PSP35Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP35Internal for PSP35Struct {
        fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                id: _id,
                value: _amount,
            });
        }

        fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, _value: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                operator: _operator,
                id: _id,
                value: _value,
            });
        }

        // Don't do cross call in test
        fn _do_safe_transfer_check(
            &mut self,
            _operator: &AccountId,
            _from: &AccountId,
            _to: &AccountId,
            _ids_amounts: &Vec<(Id, Balance)>,
            _data: &Vec<u8>,
        ) -> Result<(), PSP35Error> {
            Ok(())
        }
    }

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

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP35Error> {
            if self.return_err_on_after {
                return Err(PSP35Error::Custom(String::from("Error on _after_token_transfer")))
            }
            Ok(())
        }
    }

    impl PSP35 for PSP35Struct {}

    impl PSP35Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut PSP35Struct| {})
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP35Error> {
            self._mint_to(acc, vec![(id, amount)])
        }

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
        }
    }

    type Event = <PSP35Struct as ::ink_lang::reflect::ContractEventBase>::Type;

    #[ink::test]
    fn before_token_transfer_should_fail_transfer() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        // Can transfer tokens
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id_1, token_1_amount, vec![])
            .is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, token_id_2, token_2_amount, vec![]),
            Err(PSP35Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_transfer() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        // Can transfer tokens
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id_1, token_1_amount, vec![])
            .is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, token_id_2, token_2_amount, vec![]),
            Err(PSP35Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }

    #[ink::test]
    fn balance_of() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        // Token 1 does not exists.
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        // mint some token 1
        assert!(nft.mint(accounts.alice, token_id, 1).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id), mint_amount);

        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, None, Some(accounts.alice), token_id, mint_amount);
        assert_eq!(ink_env::test::recorded_events().count(), 1);
    }

    #[ink::test]
    fn approve() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        // no approvall exists yet
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, None), 0);
        // increase allowance
        assert!(nft.approve(accounts.bob, Some(([0; 32], 1))).is_ok());
        // allowance increased
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, Some([0; 32])), 1);
        // decrease allowance
        assert!(nft.approve(accounts.bob, Some(([0; 32], 0))).is_ok());
        // allowance decreased
        assert_eq!(!nft.allowance(accounts.alice, accounts.bob, Some([0; 32])), 0);
        // approval for all
        assert!(nft.approve(accounts.bob, None).is_ok());
        // approval for all exists
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, None), Balance::MAX);
        // approval for token exists
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, Some([0; 32])), Balance::MAX);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();

        let emmited_event = events_iter.next().unwrap();
        assert_approval_event(emmited_event, accounts.alice, accounts.bob, Some([0; 32]), 1);

        let emmited_event = events_iter.next().unwrap();
        assert_approval_event(emmited_event, accounts.alice, accounts.bob, Some([0; 32]), 0);

        let emmited_event = events_iter.next().unwrap();
        assert_approval_event(emmited_event, accounts.alice, accounts.bob, None, Balance::MAX);

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn transfer_from() {
        let token_id = [1; 32];
        let transfer_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, transfer_amount).is_ok());
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id, transfer_amount, vec![])
            .is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), transfer_amount);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, None, Some(accounts.alice), token_id, transfer_amount);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            Some(accounts.alice),
            Some(accounts.bob),
            token_id,
            transfer_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_insufficient_balance() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let transfer_amount = 2;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, token_id, transfer_amount, vec![]),
            Err(PSP35Error::InsufficientBalance),
        );
    }

    #[ink::test]
    fn transfer_from_no_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.bob, token_id, mint_amount).is_ok());
        assert_eq!(
            Err(PSP35Error::NotAllowed),
            nft.transfer_from(accounts.bob, accounts.alice, token_id, mint_amount, vec![])
        );
    }

    #[ink::test]
    fn transfer_from_with_approve() {
        let token_id = [1; 32];
        let mint_amount = 2;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert!(nft.approve(accounts.bob, Some((token_id, mint_amount))).is_ok());

        change_caller(accounts.bob);
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id, 1, vec![])
            .is_ok());

        assert_eq!(nft.balance_of(accounts.bob, token_id), 1);
        assert_eq!(nft.balance_of(accounts.alice, token_id), 1);
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, Some(token_id)), 1);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, None, Some(accounts.alice), token_id, 1);

        let emmited_event = events_iter.next().unwrap();
        assert_approval_event(emmited_event, accounts.alice, accounts.bob, Some(token_id), 2);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, Some(accounts.alice), Some(accounts.bob), token_id, 1);

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn transfer() {
        let token_id = [1; 32];
        let transfer_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, transfer_amount).is_ok());
        assert!(nft.transfer(accounts.bob, token_id, transfer_amount, vec![]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), transfer_amount);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, None, Some(accounts.alice), token_id, transfer_amount);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            Some(accounts.alice),
            Some(accounts.bob),
            token_id,
            transfer_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_insufficient_balance() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let transfer_amount = 2;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert_eq!(
            nft.transfer(accounts.bob, token_id, transfer_amount, vec![]),
            Err(PSP35Error::InsufficientBalance),
        );
    }

    #[ink::test]
    fn transfer_no_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert_eq!(
            Err(PSP35Error::NotAllowed),
            nft.transfer(accounts.bob, token_id, mint_amount, vec![])
        );
    }

    #[ink::test]
    fn transfer_with_approve() {
        let token_id = [1; 32];
        let mint_amount = 2;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert!(nft.approve(accounts.bob, Some((token_id, mint_amount))).is_ok());

        change_caller(accounts.bob);
        assert!(nft.transfer(accounts.bob, token_id, 1, vec![]).is_ok());

        assert_eq!(nft.balance_of(accounts.bob, token_id), 1);
        assert_eq!(nft.balance_of(accounts.alice, token_id), 1);
        assert_eq!(nft.allowance(accounts.alice, accounts.bob, Some(token_id)), 1);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, None, Some(accounts.alice), token_id, 1);

        let emmited_event = events_iter.next().unwrap();
        assert_approval_event(emmited_event, accounts.alice, accounts.bob, Some(token_id), 2);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(emmited_event, Some(accounts.alice), Some(accounts.bob), token_id, 1);

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn before_received_should_fail_transfer() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP35Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        // Can transfer tokens
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id_1, token_1_amount, vec![])
            .is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, token_id_2, token_2_amount, vec![]),
            Err(PSP35Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    fn assert_transfer_event(
        event: ink_env::test::EmittedEvent,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_token_id: Id,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::Transfer(Transfer { from, to, id, value }) = decoded_event {
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(id, expected_token_id, "encountered invalid Transfer.id");
            assert_eq!(value, expected_value, "encountered invalid Transfer.value");
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
    }

    fn assert_approval_event(
        event: ink_env::test::EmittedEvent,
        expected_owner: AccountId,
        expected_operator: AccountId,
        expected_id: Option<Id>,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::Approval(Approval {
            owner,
            operator,
            id,
            value,
        }) = decoded_event
        {
            assert_eq!(owner, expected_owner, "encountered invalid Approval.owner");
            assert_eq!(operator, expected_operator, "encountered invalid Approval.to");
            assert_eq!(id, expected_id, "encountered invalid Approval.id");
            assert_eq!(value, expected_value, "encountered invalid Approval.value");
        } else {
            panic!("encountered unexpected event kind: expected a Approval event")
        }
    }
}
