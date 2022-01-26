#![feature(min_specialization)]
#[cfg(feature = "psp1155")]
#[brush::contract]
mod psp1155 {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp1155::*;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;

    #[ink(event)]
    pub struct TransferSingle {
        operator: AccountId,
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        value: Balance,
    }

    #[ink(event)]
    pub struct TransferBatch {
        operator: AccountId,
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        ids_amounts: Vec<(Id, Balance)>,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP1155Internal for PSP1155Struct {
        fn _emit_transfer_single_event(
            &self,
            _operator: AccountId,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
            _amount: Balance,
        ) {
            self.env().emit_event(TransferSingle {
                operator: _operator,
                from: _from,
                to: _to,
                id: _id,
                value: _amount,
            });
        }

        fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner: _owner,
                operator: _operator,
                approved: _approved,
            });
        }

        fn _emit_transfer_batch_event(
            &self,
            _operator: AccountId,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _ids_amounts: Vec<(Id, Balance)>,
        ) {
            self.env().emit_event(TransferBatch {
                operator: _operator,
                from: _from,
                to: _to,
                ids_amounts: _ids_amounts,
            });
        }

        // Don't do cross call in test
        fn _do_safe_transfer_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _ids_amounts: Vec<(Id, Balance)>,
            _data: Vec<u8>,
        ) -> Result<(), PSP1155Error> {
            Ok(())
        }

        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if self.return_err_on_before {
                return Err(PSP1155Error::Custom(String::from("Error on _before_token_transfer")));
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if self.return_err_on_after {
                return Err(PSP1155Error::Custom(String::from("Error on _after_token_transfer")));
            }
            Ok(())
        }
    }

    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint_to(acc, vec![(id, amount)])
        }

        #[ink(message)]
        pub fn change_state_err(&mut self) {
            if self.return_err_on_before {
                self.return_err_on_before = false;
                self.return_err_on_after = true;
            } else {
                self.return_err_on_before = true;
            }
        }
    }

    type Event = <PSP1155Struct as ::ink_lang::BaseEvent>::Type;

    #[ink::test]
    fn balance_of() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        // Token 1 does not exists.
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        // mint some token 1
        assert!(nft.mint(accounts.alice, token_id, 1).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id), mint_amount);

        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id,
            mint_amount,
        );
        assert_eq!(ink_env::test::recorded_events().count(), 1);
    }

    #[ink::test]
    fn balance_of_batch() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        let accounts_ids = vec![(accounts.alice, token_id_1), (accounts.alice, token_id_2)];
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        // Token 1 does not exists.
        assert_eq!(nft.balance_of_batch(accounts_ids.clone()), vec![0, 0]);
        // mint some token 1
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert_eq!(nft.balance_of_batch(accounts_ids.clone()), vec![token_1_amount, 0]);

        // mint some token 2
        assert!(nft.mint(accounts.bob, token_id_2, token_2_amount).is_ok());
        assert_eq!(
            nft.balance_of_batch(vec![(accounts.alice, token_id_1), (accounts.bob, token_id_2)]),
            vec![token_1_amount, token_2_amount]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id_1,
            token_1_amount,
        );
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.bob),
            token_id_2,
            token_2_amount,
        );
        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn set_approval_for_all() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        // no approvall exists yet
        assert!(!nft.is_approved_for_all(accounts.alice, accounts.bob));
        // set approval
        assert!(nft.set_approval_for_all(accounts.bob, true).is_ok());
        // approval exists
        assert!(nft.is_approved_for_all(accounts.alice, accounts.bob));
        // remove approval
        assert!(nft.set_approval_for_all(accounts.bob, false).is_ok());
        // no approvall exists
        assert!(!nft.is_approved_for_all(accounts.alice, accounts.bob));

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, false);

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_single() {
        let token_id = [1; 32];
        let transfer_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, transfer_amount).is_ok());
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id, transfer_amount, vec![])
            .is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), transfer_amount);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id,
            transfer_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            Some(accounts.alice),
            Some(accounts.bob),
            token_id,
            transfer_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn transfer_from_batch() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amount = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());

        assert_eq!(
            nft.balance_of_batch(vec![(accounts.bob, token_id_1), (accounts.bob, token_id_2)]),
            vec![0, 0]
        );
        assert_eq!(
            nft.balance_of_batch(vec![(accounts.alice, token_id_1), (accounts.alice, token_id_2)]),
            amounts.clone()
        );

        assert!(nft
            .batch_transfer_from(accounts.alice, accounts.bob, ids_amount.clone(), vec![],)
            .is_ok());

        assert_eq!(
            nft.balance_of_batch(vec![(accounts.bob, token_id_1), (accounts.bob, token_id_2)]),
            amounts.clone()
        );
        assert_eq!(
            nft.balance_of_batch(vec![(accounts.alice, token_id_1), (accounts.alice, token_id_2)]),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id_1,
            token_1_amount,
        );
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id_2,
            token_2_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_batch_event(
            emmited_event,
            accounts.alice,
            Some(accounts.alice),
            Some(accounts.bob),
            &ids_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn transfer_from_single_insufficient_balance() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let transfer_amount = 2;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert_eq!(
            Err(PSP1155Error::InsufficientBalance),
            nft.transfer_from(accounts.alice, accounts.bob, token_id, transfer_amount, vec![])
        );
    }

    #[ink::test]
    fn transfer_from_single_no_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.bob, token_id, mint_amount).is_ok());
        assert_eq!(
            Err(PSP1155Error::NotAllowed),
            nft.transfer_from(accounts.bob, accounts.alice, token_id, mint_amount, vec![])
        );
    }

    #[ink::test]
    fn transfer_from_single_with_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, mint_amount).is_ok());
        assert!(nft.set_approval_for_all(accounts.bob, true).is_ok());

        change_caller(accounts.bob);
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, token_id, mint_amount, vec![])
            .is_ok());

        assert_eq!(nft.balance_of(accounts.bob, token_id), mint_amount);
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id,
            mint_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.bob,
            Some(accounts.alice),
            Some(accounts.bob),
            token_id,
            mint_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    fn transfer_from_batch_insufficient_balance() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, 2), (token_id_2, 21)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        assert_eq!(
            Err(PSP1155Error::InsufficientBalance),
            nft.batch_transfer_from(accounts.alice, accounts.bob, ids_amounts, vec![])
        );
    }

    #[ink::test]
    fn transfer_from_batch_no_approve() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.bob, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.bob, token_id_2, token_2_amount).is_ok());

        assert_eq!(
            Err(PSP1155Error::NotAllowed),
            nft.batch_transfer_from(accounts.bob, accounts.alice, ids_amounts, vec![],)
        );
    }

    #[ink::test]
    fn transfer_from_batch_with_approve() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids_amounts = vec![(token_id_1, token_1_amount), (token_id_2, token_2_amount)];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        assert!(nft.set_approval_for_all(accounts.bob, true).is_ok());

        change_caller(accounts.bob);
        assert!(nft
            .batch_transfer_from(accounts.alice, accounts.bob, ids_amounts.clone(), vec![])
            .is_ok());

        assert_eq!(
            nft.balance_of_batch(vec![(accounts.bob, token_id_1), (accounts.bob, token_id_2)]),
            amounts.clone()
        );

        assert_eq!(
            nft.balance_of_batch(vec![(accounts.alice, token_id_1), (accounts.alice, token_id_2)]),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id_1,
            token_1_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            None,
            Some(accounts.alice),
            token_id_2,
            token_2_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_batch_event(
            emmited_event,
            accounts.bob,
            Some(accounts.alice),
            Some(accounts.bob),
            &ids_amounts,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 4);
    }

    #[ink::test]
    fn before_and_after_token_transfer_should_fail_transfer() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        // Can transfer tokens
        assert!(nft.transfer_from(accounts.alice, accounts.bob, token_id_1, token_1_amount, vec![]).is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, token_id_2, token_2_amount, vec![]),
            Err(PSP1155Error::Custom(String::from("Error on _before_token_transfer")))
        );
        // Turn on error on _after_token_transfer
        nft.change_state_err();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, token_id_2, token_2_amount, vec![]),
            Err(PSP1155Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }

    fn assert_transfer_event(
        event: ink_env::test::EmittedEvent,
        expected_operator: AccountId,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_token_id: Id,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::TransferSingle(TransferSingle {
            operator,
            from,
            to,
            id,
            value,
        }) = decoded_event
        {
            assert_eq!(
                operator, expected_operator,
                "encountered invalid TransferSingle.operator"
            );
            assert_eq!(from, expected_from, "encountered invalid TransferSingle.from");
            assert_eq!(to, expected_to, "encountered invalid TransferSingle.to");
            assert_eq!(id, expected_token_id, "encountered invalid TransferSingle.id");
            assert_eq!(value, expected_value, "encountered invalid TransferSingle.value");
        } else {
            panic!("encountered unexpected event kind: expected a TransferSingle event")
        }
    }

    fn assert_transfer_batch_event(
        event: ink_env::test::EmittedEvent,
        expected_operator: AccountId,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_token_ids_and_values: &[(Id, Balance)],
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::TransferBatch(TransferBatch {
            operator,
            from,
            to,
            ids_amounts,
        }) = decoded_event
        {
            assert_eq!(
                operator, expected_operator,
                "encountered invalid TransferBatch.operator"
            );
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

    fn assert_approve_event(
        event: ink_env::test::EmittedEvent,
        expected_owner: AccountId,
        expected_operator: AccountId,
        expected_approved: bool,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::ApprovalForAll(ApprovalForAll {
            owner,
            operator,
            approved,
        }) = decoded_event
        {
            assert_eq!(owner, expected_owner, "encountered invalid ApprovalForAll.owner");
            assert_eq!(operator, expected_operator, "encountered invalid ApprovalForAll.to");
            assert_eq!(
                approved, expected_approved,
                "encountered invalid ApprovalForAll.approved"
            );
        } else {
            panic!("encountered unexpected event kind: expected a ApprovalForAll event")
        }
    }
}
