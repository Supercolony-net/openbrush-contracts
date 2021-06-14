#[cfg(test)]
#[ink_lang::contract]
mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use crate::impls::{Erc20, Erc20Storage};
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        Lazy,
    };
    use ink_prelude::{string::{String}};
    use ink_lang as ink;
    use brush::{
        traits::{InkStorage},
        iml_getters,
    };
    use ink::{Env, EmitEvent};
    use ink_env::{
        hash::{
            Blake2x256,
            CryptoHash,
            HashOutput,
        },
        Clear,
    };
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

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20Struct {
        total_supply: Lazy<Balance>,
        balances: StorageHashMap<AccountId, Balance>,
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        name: Lazy<Option<String>>,
        symbol: Lazy<Option<String>>,
        decimal: Lazy<u8>,
    }
    type Event = <Erc20Struct as ::ink_lang::BaseEvent>::Type;

    impl InkStorage for Erc20Struct {}
    impl Erc20Storage for Erc20Struct {
        iml_getters!(total_supply, _supply, _supply_mut, Lazy<Balance>);
        iml_getters!(balances, _balances, _balances_mut, StorageHashMap<AccountId, Balance>);
        iml_getters!(allowances, _allowances, _allowances_mut, StorageHashMap<(AccountId, AccountId), Balance>);
        iml_getters!(name, _name, _name_mut, Lazy<Option<String>>);
        iml_getters!(symbol, _symbol, _symbol_mut, Lazy<Option<String>>);
        iml_getters!(decimal, _decimals, _decimals_mut, Lazy<u8>);
    }
    impl Erc20 for Erc20Struct {
        fn emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        fn emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                spender: _spender,
                value: _amount,
            });
        }
    }

    impl Erc20Struct {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance) -> Self {
            let mut instance = Self::_empty();
            instance.mint(instance.env().caller(), _total_supply);
            instance
        }

        #[ink(message)]
        pub fn temp(&self){}
    }

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
        fn encoded_into_hash<T>(entity: &T) -> Hash
            where
                T: scale::Encode,
        {
            let mut result = Hash::clear();
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result
            }
            let mut hash_output =
                <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"Erc20Struct::Transfer",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20Struct::Transfer::from",
                value: &expected_from,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20Struct::Transfer::to",
                value: &expected_to,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20Struct::Transfer::value",
                value: &expected_value,
            }),
        ];
        for (n, (actual_topic, expected_topic)) in
        event.topics.iter().zip(expected_topics).enumerate()
        {
            let topic = actual_topic
                .decode::<Hash>()
                .expect("encountered invalid topic encoding");
            assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
        }
    }

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        // Constructor works.
        let _erc20 = Erc20Struct::new(100);

        // Transfer event triggered during initial construction.
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(1, emitted_events.len());

        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            100,
        );
    }

    /// The total supply was applied.
    #[ink::test]
    fn total_supply_works() {
        // Constructor works.
        let erc20 = Erc20Struct::new(100);
        // Transfer event triggered during initial construction.
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            100,
        );
        // Get the token total supply.
        assert_eq!(erc20.total_supply(), 100);
    }

    /// Get the actual balance of an account.
    #[ink::test]
    fn balance_of_works() {
        // Constructor works
        let erc20 = Erc20Struct::new(100);
        // Transfer event triggered during initial construction
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            100,
        );
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
        // Alice owns all the tokens on deployment
        assert_eq!(erc20.balance_of(accounts.alice), 100);
        // Bob does not owns tokens
        assert_eq!(erc20.balance_of(accounts.bob), 0);
    }

    #[ink::test]
    fn transfer_works() {
        // Constructor works.
        let mut erc20 = Erc20Struct::new(100);
        // Transfer event triggered during initial construction.
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        assert_eq!(erc20.balance_of(accounts.bob), 0);
        // Alice transfers 10 tokens to Bob.
        erc20.transfer(accounts.bob, 10);
        // Bob owns 10 tokens.
        assert_eq!(erc20.balance_of(accounts.bob), 10);

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events.len(), 2);
        // Check first transfer event related to ERC-20 instantiation.
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            100,
        );
        // Check the second transfer event relating to the actual trasfer.
        assert_transfer_event(
            &emitted_events[1],
            Some(AccountId::from([0x01; 32])),
            Some(AccountId::from([0x02; 32])),
            10,
        );
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn invalid_transfer_should_fail() {
        // Constructor works.
        let mut erc20 = Erc20Struct::new(100);
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        assert_eq!(erc20.balance_of(accounts.bob), 0);
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or([0x0; 32].into());
        // Create call
        let mut data =
            ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );

        // Bob fails to transfers 10 tokens to Eve.
        erc20.transfer(accounts.eve, 10);
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientAllowance")]
    fn transfer_from_fails() {
        // Constructor works.
        let mut erc20 = Erc20Struct::new(100);
        // Transfer event triggered during initial construction.
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        // Bob fails to transfer tokens owned by Alice.
        erc20.transfer_from(accounts.alice, accounts.eve, 10);
    }

    #[ink::test]
    fn transfer_from_works() {
        // Constructor works.
        let mut erc20 = Erc20Struct::new(100);
        // Transfer event triggered during initial construction.
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        // Alice approves Bob for token transfers on her behalf.
        erc20.approve(accounts.bob, 10);

        // The approve event takes place.
        assert_eq!(ink_env::test::recorded_events().count(), 2);

        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or([0x0; 32].into());
        // Create call.
        let mut data =
            ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );

        // Bob transfers tokens from Alice to Eve.
        erc20.transfer_from(accounts.alice, accounts.eve, 10);
        // Eve owns tokens.
        assert_eq!(erc20.balance_of(accounts.eve), 10);

        // Check all transfer events that happened during the previous calls:
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events.len(), 4);
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            100,
        );
        // The second event `emitted_events[1]` is an Approve event that we skip checking.
        assert_transfer_event(
            &emitted_events[2],
            Some(AccountId::from([0x01; 32])),
            Some(AccountId::from([0x05; 32])),
            10,
        );
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn allowance_must_not_change_on_failed_transfer() {
        let mut erc20 = Erc20Struct::new(100);
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        // Alice approves Bob for token transfers on her behalf.
        let alice_balance = erc20.balance_of(accounts.alice);
        let initial_allowance = alice_balance + 2;
        erc20.approve(accounts.bob, initial_allowance);

        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
            .unwrap_or([0x0; 32].into());
        // Create call.
        let mut data =
            ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );

        erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1);
    }

    /// For calculating the event topic hash.
    struct PrefixedValue<'a, 'b, T> {
        pub prefix: &'a [u8],
        pub value: &'b T,
    }

    impl<X> scale::Encode for PrefixedValue<'_, '_, X>
        where
            X: scale::Encode,
    {
        #[inline]
        fn size_hint(&self) -> usize {
            self.prefix.size_hint() + self.value.size_hint()
        }

        #[inline]
        fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
            self.prefix.encode_to(dest);
            self.value.encode_to(dest);
        }
    }
}