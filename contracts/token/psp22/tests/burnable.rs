#[brush::contract]
mod psp22_burnable {
    use brush::test_utils::*;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;
    use psp22::extensions::burnable::*;
    /// Imports all the definitions from the outer scope so we can use them here.
    use psp22::traits::*;
    use std::panic;

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// A simple PSP-20 contract.
    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct PSP22Struct {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    type Event = <PSP22Struct as ::ink_lang::BaseEvent>::Type;

    impl PSP22 for PSP22Struct {
        fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        // Override these functions with an empty body to omit error (cross-contract calls are not supported in off-chain environment)
        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {}

        fn _do_safe_transfer_check(&self, _from: AccountId, _to: AccountId, _value: Balance, _data: Vec<u8>) {}
    }

    impl PSP22Struct {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance) -> Self {
            let mut instance = Self::default();
            instance._mint(instance.env().caller(), _total_supply);
            instance
        }
    }

    impl PSP22Burnable for PSP22Struct {}

    fn assert_transfer_event(
        event: &ink_env::test::EmittedEvent,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"PSP22Struct::Transfer",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"PSP22Struct::Transfer::from",
                value: &expected_from,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"PSP22Struct::Transfer::to",
                value: &expected_to,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"PSP22Struct::Transfer::value",
                value: &expected_value,
            }),
        ];
        for (n, (actual_topic, expected_topic)) in event.topics.iter().zip(expected_topics).enumerate() {
            let topic = actual_topic
                .decode::<Hash>()
                .expect("encountered invalid topic encoding");
            assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
        }
    }

    // Testing burnable extension
    #[ink::test]
    #[should_panic(expected = "ZeroRecipientAddress")]
    fn should_not_burn_from_zero_address() {
        let mut psp22 = PSP22Struct::new(100);
        let amount_to_burn = 10;
        psp22.approve(AccountId::from([0; 32]), amount_to_burn);

        psp22.burn_from(AccountId::from([0; 32]), amount_to_burn);
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn should_not_burn_if_burn_amount_greater_then_account_balance() {
        let initial_balance = 10;
        let mut psp22 = PSP22Struct::new(initial_balance);
        let amount_to_burn = 100;

        psp22.burn(amount_to_burn);
    }

    #[ink::test]
    fn should_emit_transfer_event_after_burn() {
        // Constructor works.
        let initial_amount = 100;
        let mut psp22 = PSP22Struct::new(initial_amount);
        // Transfer event triggered during initial construction.
        let amount_to_burn = 10;

        psp22.burn(amount_to_burn);

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events.len(), 2);
        // Check first transfer event related to PSP-20 instantiation.
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            initial_amount,
        );
        // Check the second transfer event relating to the actual transfer.
        assert_transfer_event(
            &emitted_events[1],
            Some(AccountId::from([0x01; 32])),
            None,
            amount_to_burn,
        );
    }

    #[ink::test]
    fn total_supply_decreases_after_burning() {
        let mut psp22 = PSP22Struct::new(100);

        // Account's total supply before burning
        let total_supply = psp22.total_supply();
        let amount_to_burn = 10;

        psp22.burn(amount_to_burn);

        // Account's total supply after burning
        let new_total_supply = psp22.total_supply();

        assert_eq!(new_total_supply, total_supply - amount_to_burn);
    }

    #[ink::test]
    fn burn_requested_amount() {
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        // Alice's balance before burning
        let alice_balance = psp22.balance_of(accounts.alice);
        let amount_to_burn = 10;

        psp22.burn(amount_to_burn);

        // Alice's balance after burning
        let new_alice_balance = psp22.balance_of(accounts.alice);

        assert_eq!(new_alice_balance, alice_balance - amount_to_burn);
    }

    #[ink::test]
    fn burn_requested_amount_from() {
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let amount_to_burn = 50;

        // Alice approves Bob for token transfers on her behalf.
        psp22.approve(accounts.bob, amount_to_burn);
        let alice_balance = psp22.balance_of(accounts.alice);

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

        // Burning some amount from Alice's account
        psp22.burn_from(accounts.alice, amount_to_burn);

        // Expecting Alice's balance decrease
        assert_eq!(psp22.balance_of(accounts.alice), alice_balance - amount_to_burn);
    }

    #[ink::test]
    fn allowance_must_decrease_after_burning_from() {
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let allowed_amount_to_burn = 50;
        let amount_to_burn = 40;

        // Alice approves Bob for token transfers on her behalf.
        psp22.approve(accounts.bob, allowed_amount_to_burn);
        assert_eq!(psp22.allowance(accounts.alice, accounts.bob), allowed_amount_to_burn);

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

        // Burning from Alice's account
        psp22.burn_from(accounts.alice, amount_to_burn);

        // Expecting Bob's allowance to spend Alice's tokens decrease
        assert_eq!(
            psp22.allowance(accounts.alice, accounts.bob),
            allowed_amount_to_burn - amount_to_burn
        );
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientAllowance")]
    fn burn_from_fails_if_amount_exceeds_allowance() {
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let amount_to_burn = 50;

        // Transfer to Bob some amount so it can be burnt
        psp22.transfer(accounts.bob, amount_to_burn, Vec::<u8>::new());
        // Alice's allowance to spend Bob's tokens is 0
        assert_eq!(psp22.allowance(accounts.bob, accounts.alice), 0);
        // Try to burn some amount from Bob's account
        psp22.burn_from(accounts.bob, amount_to_burn);
    }
}