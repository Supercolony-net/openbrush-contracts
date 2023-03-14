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
#[cfg(feature = "psp37")]
#[openbrush::contract]
mod psp37_batch {
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        prelude::vec::Vec,
    };
    use openbrush::{
        test_utils::{
            accounts,
            change_caller,
        },
        traits::Storage,
    };
    use openbrush_contracts::psp37::extensions::batch::*;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        value: Balance,
    }

    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        ids_amounts: Vec<(Id, Balance)>,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        id: Option<Id>,
        value: Balance,
    }

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct PSP37Struct {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl psp37::Internal for PSP37Struct {
        fn _emit_approval_event(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>, _value: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                operator: _operator,
                id: _id,
                value: _value,
            });
        }

        fn _emit_transfer_batch_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _ids_amounts: Vec<(Id, Balance)>,
        ) {
            self.env().emit_event(TransferBatch {
                from: _from,
                to: _to,
                ids_amounts: _ids_amounts,
            });
        }
    }

    impl PSP37 for PSP37Struct {}

    impl PSP37Batch for PSP37Struct {}

    impl PSP37Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            self._mint_to(acc, ids_amounts)
        }
    }

    type Event = <PSP37Struct as ::ink::reflect::ContractEventBase>::Type;

    #[ink::test]
    fn batch_transfer() {
        let token_id1 = Id::U128(1);
        let token_id2 = Id::U128(2);
        let id_1_amount = 1;
        let id_2_amount = 20;
        let ids_amounts = vec![(token_id1.clone(), id_1_amount), (token_id2.clone(), id_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, None), 2);

        assert!(nft.batch_transfer(accounts.bob, ids_amounts.clone(), vec![]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, Some(token_id1.clone())), 0);
        assert_eq!(nft.balance_of(accounts.alice, Some(token_id2.clone())), 0);

        assert_eq!(nft.balance_of(accounts.bob, Some(token_id1)), id_1_amount);
        assert_eq!(nft.balance_of(accounts.bob, Some(token_id2)), id_2_amount);

        assert_eq!(nft.balance_of(accounts.alice, None), 0);
        assert_eq!(nft.balance_of(accounts.bob, None), 2);

        // EVENTS ASSERTS
        let mut events_iter = ink::env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_batch_transfer_event(emmited_event, None, Some(accounts.alice), &ids_amounts);

        let emmited_event = events_iter.next().unwrap();
        assert_batch_transfer_event(emmited_event, Some(accounts.alice), Some(accounts.bob), &ids_amounts);

        assert_eq!(ink::env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_batch_from() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![
            (token_id_1.clone(), token_1_amount),
            (token_id_2.clone(), token_2_amount),
        ];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());

        assert_eq!(nft.balance_of(accounts.bob, Some(token_id_1.clone())), 0);
        assert_eq!(nft.balance_of(accounts.bob, Some(token_id_2.clone())), 0);

        assert_eq!(nft.balance_of(accounts.alice, Some(token_id_1.clone())), amounts[0]);
        assert_eq!(nft.balance_of(accounts.alice, Some(token_id_2.clone())), amounts[1]);

        assert_eq!(nft.balance_of(accounts.alice, None), 2);
        assert_eq!(nft.balance_of(accounts.bob, None), 0);

        assert!(nft
            .batch_transfer_from(accounts.alice, accounts.bob, ids_amounts.clone(), vec![])
            .is_ok());

        assert_eq!(nft.balance_of(accounts.bob, Some(token_id_1.clone())), amounts[0]);
        assert_eq!(nft.balance_of(accounts.bob, Some(token_id_2.clone())), amounts[1]);

        assert_eq!(nft.balance_of(accounts.alice, Some(token_id_1.clone())), 0);
        assert_eq!(nft.balance_of(accounts.alice, Some(token_id_2.clone())), 0);

        assert_eq!(nft.balance_of(accounts.alice, None), 0);
        assert_eq!(nft.balance_of(accounts.bob, None), 2);
    }

    #[ink::test]
    fn batch_transfer_from_insufficient_balance() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());
        assert_eq!(
            nft.batch_transfer_from(
                accounts.alice,
                accounts.bob,
                ids_amounts
                    .iter()
                    .map(|(_, amount)| { (Id::U128(123), *amount) })
                    .collect(),
                vec![]
            ),
            Err(PSP37Error::InsufficientBalance),
        );
    }

    #[ink::test]
    fn batch_transfer_from_no_approve() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.bob, ids_amounts.clone()).is_ok());

        assert_eq!(
            nft.batch_transfer_from(accounts.bob, accounts.alice, ids_amounts, vec![]),
            Err(PSP37Error::NotAllowed)
        );
    }

    #[ink::test]
    fn batch_transfer_with_approve() {
        let token_id_1 = Id::U128(1);
        let token_id_2 = Id::U128(2);
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![
            (token_id_1.clone(), token_1_amount),
            (token_id_2.clone(), token_2_amount),
        ];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP37Struct::new();
        assert!(nft.mint(accounts.alice, ids_amounts.clone()).is_ok());
        assert!(nft.approve(accounts.bob, None, Balance::MAX).is_ok());

        change_caller(accounts.bob);
        assert!(nft
            .batch_transfer_from(accounts.alice, accounts.bob, ids_amounts.clone(), vec![])
            .is_ok());

        assert_eq!(nft.balance_of(accounts.bob, Some(token_id_1.clone())), amounts[0]);
        assert_eq!(nft.balance_of(accounts.bob, Some(token_id_2.clone())), amounts[1]);
        assert_eq!(nft.balance_of(accounts.alice, Some(token_id_1)), 0);
        assert_eq!(nft.balance_of(accounts.alice, Some(token_id_2)), 0);

        assert_eq!(nft.balance_of(accounts.alice, None), 0);
        assert_eq!(nft.balance_of(accounts.bob, None), 2);

        // EVENTS ASSERTS
        let mut events_iter = ink::env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_batch_transfer_event(emmited_event, None, Some(accounts.alice), &ids_amounts);

        let emmited_event = events_iter.next().unwrap();
        assert_approval_event(emmited_event, accounts.alice, accounts.bob, None, Balance::MAX);

        let emmited_event = events_iter.next().unwrap();
        assert_batch_transfer_event(emmited_event, Some(accounts.alice), Some(accounts.bob), &ids_amounts);

        assert_eq!(ink::env::test::recorded_events().count(), 3);
    }

    fn assert_batch_transfer_event(
        event: ink::env::test::EmittedEvent,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_token_ids_and_values: &[(Id, Balance)],
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::TransferBatch(TransferBatch { from, to, ids_amounts }) = decoded_event {
            assert_eq!(from, expected_from, "encountered invalid TransferBatch.from");
            assert_eq!(to, expected_to, "encountered invalid TransferBatch.to");
            assert_eq!(
                ids_amounts, expected_token_ids_and_values,
                "encountered invalid TransferBatch.ids_amounts"
            );
        } else {
            panic!("encountered unexpected event kind: expected a TransferBatch event")
        }
    }

    fn assert_approval_event(
        event: ink::env::test::EmittedEvent,
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
