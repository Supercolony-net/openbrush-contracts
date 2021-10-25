#[cfg(test)]
#[brush::contract]
mod tests {
    use brush::traits::ZERO_ADDRESS;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;
    use psp1155::traits::*;

    #[ink(event)]
    pub struct TransferSingle {
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        id: Id,
        value: Balance,
    }

    #[ink(event)]
    pub struct TransferBatch {
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        ids: Vec<Id>,
        values: Vec<Balance>,
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
    }

    impl IPSP1155 for PSP1155Struct {
        fn _emit_transfer_single_event(
            &self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
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
            _from: AccountId,
            _to: AccountId,
            _ids: Vec<Id>,
            _amounts: Vec<Balance>,
        ) {
            self.env().emit_event(TransferBatch {
                operator: _operator,
                from: _from,
                to: _to,
                ids: _ids,
                values: _amounts,
            });
        }

        // Don't do cross call in test
        fn _do_safe_transfer_acceptance_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _amount: Balance,
            _data: Vec<u8>,
        ) -> Result<(), psp1155::traits::PSP1155Error> {
            Ok(())
        }

        // Don't do cross call in test
        fn _do_batch_safe_transfer_acceptance_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _ids: Vec<Id>,
            _amounts: Vec<Balance>,
            _data: Vec<u8>,
        ) -> Result<(), psp1155::traits::PSP1155Error> {
            Ok(())
        }
    }

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    type Event = <PSP1155Struct as ::ink_lang::BaseEvent>::Type;

    #[ink::test]
    fn balance_of() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        // Token 1 does not exists.
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        // mint some token 1
        nft._mint(accounts.alice, token_id, 1);
        assert_eq!(nft.balance_of(accounts.alice, token_id), mint_amount);

        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
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
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        // Token 1 does not exists.
        assert_eq!(
            nft.balance_of_batch(vec![accounts.alice, accounts.alice], vec![token_id_1, token_id_2],),
            vec![0, 0]
        );
        // mint some token 1
        nft._mint(accounts.alice, token_id_1, token_1_amount);
        assert_eq!(
            nft.balance_of_batch(vec![accounts.alice, accounts.alice], vec![token_id_1, token_id_2],),
            vec![token_1_amount, 0]
        );

        // mint some token 2
        nft._mint(accounts.bob, token_id_2, token_2_amount);
        assert_eq!(
            nft.balance_of_batch(vec![accounts.alice, accounts.bob], vec![token_id_1, token_id_2],),
            vec![token_1_amount, token_2_amount]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.bob,
            token_id_2,
            token_2_amount,
        );
        assert_eq!(ink_env::test::recorded_events().count(), 2);
    }

    #[ink::test]
    fn set_approval_for_all() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        // no approvall exists yet
        assert!(!nft.is_approved_for_all(accounts.alice, accounts.bob),);
        // set approval
        nft.set_approval_for_all(accounts.bob, true);
        // approval exists
        assert!(nft.is_approved_for_all(accounts.alice, accounts.bob));
        // remove approval
        nft.set_approval_for_all(accounts.bob, false);
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
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.alice, token_id, transfer_amount);
        nft.safe_transfer_from(accounts.alice, accounts.bob, token_id, transfer_amount, [].to_vec());
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), transfer_amount);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id,
            transfer_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            accounts.alice,
            accounts.bob,
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
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.alice, token_id_1, token_1_amount);
        nft._mint(accounts.alice, token_id_2, token_2_amount);
        nft.safe_batch_transfer_from(accounts.alice, accounts.bob, ids.clone(), amounts.clone(), [].to_vec());

        assert_eq!(
            nft.balance_of_batch(vec![accounts.bob, accounts.bob], ids.clone()),
            amounts.clone()
        );
        assert_eq!(
            nft.balance_of_batch(vec![accounts.alice, accounts.alice], ids.clone()),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_2,
            token_2_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_batch_event(
            emmited_event,
            accounts.alice,
            accounts.alice,
            accounts.bob,
            &ids,
            &amounts,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn transfer_from_single_insufficient_balance() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let transfer_amount = 2;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.alice, token_id, mint_amount);
        nft.safe_transfer_from(accounts.alice, accounts.bob, token_id, transfer_amount, [].to_vec());
    }

    #[ink::test]
    #[should_panic(expected = "ApproveRequired")]
    fn transfer_from_single_no_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.bob, token_id, mint_amount);
        nft.safe_transfer_from(accounts.bob, accounts.alice, token_id, mint_amount, [].to_vec());
    }

    #[ink::test]
    fn transfer_from_single_with_approve() {
        let token_id = [1; 32];
        let mint_amount = 1;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.alice, token_id, mint_amount);
        nft.set_approval_for_all(accounts.bob, true);

        // CHANGE CALLEE MANUALLY
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
        // Create call.
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );
        nft.safe_transfer_from(accounts.alice, accounts.bob, token_id, mint_amount, [].to_vec());

        assert_eq!(nft.balance_of(accounts.bob, token_id), mint_amount);
        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id,
            mint_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.bob,
            accounts.alice,
            accounts.bob,
            token_id,
            mint_amount,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 3);
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn transfer_from_batch_insufficient_balance() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let wrong_amounts = vec![2, 21];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.alice, token_id_1, token_1_amount);
        nft._mint(accounts.alice, token_id_2, token_2_amount);
        nft.safe_batch_transfer_from(
            accounts.alice,
            accounts.bob,
            ids.clone(),
            wrong_amounts.clone(),
            [].to_vec(),
        );
    }

    #[ink::test]
    #[should_panic(expected = "ApproveRequired")]
    fn transfer_from_batch_no_approve() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.bob, token_id_1, token_1_amount);
        nft._mint(accounts.bob, token_id_2, token_2_amount);

        nft.safe_batch_transfer_from(accounts.bob, accounts.alice, ids.clone(), amounts.clone(), [].to_vec());
    }

    #[ink::test]
    fn transfer_from_batch_with_approve() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let ids = vec![token_id_1, token_id_2];
        let amounts = vec![token_1_amount, token_2_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        nft._mint(accounts.alice, token_id_1, token_1_amount);
        nft._mint(accounts.alice, token_id_2, token_2_amount);
        nft.set_approval_for_all(accounts.bob, true);

        // CHANGE CALLEE MANUALLY
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
        // Create call.
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );
        nft.safe_batch_transfer_from(accounts.alice, accounts.bob, ids.clone(), amounts.clone(), [].to_vec());

        assert_eq!(
            nft.balance_of_batch(vec![accounts.bob, accounts.bob], ids.clone()),
            amounts.clone()
        );

        assert_eq!(
            nft.balance_of_batch(vec![accounts.alice, accounts.alice], ids.clone()),
            vec![0, 0]
        );

        // EVENTS ASSERTS
        let mut events_iter = ink_env::test::recorded_events();
        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_1,
            token_1_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_event(
            emmited_event,
            accounts.alice,
            ZERO_ADDRESS.into(),
            accounts.alice,
            token_id_2,
            token_2_amount,
        );

        let emmited_event = events_iter.next().unwrap();
        assert_approve_event(emmited_event, accounts.alice, accounts.bob, true);

        let emmited_event = events_iter.next().unwrap();
        assert_transfer_batch_event(
            emmited_event,
            accounts.bob,
            accounts.alice,
            accounts.bob,
            &ids,
            &amounts,
        );

        assert_eq!(ink_env::test::recorded_events().count(), 4);
    }

    fn assert_transfer_event(
        event: ink_env::test::EmittedEvent,
        expected_operator: AccountId,
        expected_from: AccountId,
        expected_to: AccountId,
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
        expected_from: AccountId,
        expected_to: AccountId,
        expected_token_ids: &[Id],
        expected_values: &[Balance],
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::TransferBatch(TransferBatch {
            operator,
            from,
            to,
            ids,
            values,
        }) = decoded_event
        {
            assert_eq!(
                operator, expected_operator,
                "encountered invalid TransferBatch.operator"
            );
            assert_eq!(from, expected_from, "encountered invalid TransferBatch.from");
            assert_eq!(to, expected_to, "encountered invalid TransferBatch.to");
            assert_eq!(ids, expected_token_ids, "encountered invalid TransferBatch.ids");
            assert_eq!(values, expected_values, "encountered invalid TransferBatch.values");
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
