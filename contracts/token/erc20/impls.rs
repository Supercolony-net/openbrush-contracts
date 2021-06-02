use crate::traits::{
    Erc20Error,
};
use ink_storage::{
    collections::{
        HashMap as StorageHashMap,
    },
    Lazy,
};
use utils::{
    traits::{InkStorage, AccountId, Balance},
    define_getters,
};
use ink_prelude::{string::String};

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Erc20Storage: InkStorage {
    define_getters!(_supply, _supply_mut, Lazy<Balance>);
    define_getters!(_balances, _balances_mut, StorageHashMap<AccountId, Balance>);
    define_getters!(_allowances, _allowances_mut, StorageHashMap<(AccountId, AccountId), Balance>);
    define_getters!(_name, _name_mut, Lazy<Option<String>>);
    define_getters!(_symbol, _symbol_mut, Lazy<Option<String>>);
    define_getters!(_decimals, _decimals_mut, Lazy<u8>);
}

pub trait Erc20Internal: Erc20Storage {
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance);

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);

    fn _balance_of(&self, owner: AccountId) -> Balance {
        self._balances().get(&owner).copied().unwrap_or(0)
    }

    fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) -> Result<(), Erc20Error> {
        Ok(())
    }

    fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), Erc20Error> {
        if from == ZERO_ADDRESS.into() {
            return Err(Erc20Error::ZeroSenderAddress);
        }
        if to == ZERO_ADDRESS.into() {
            return Err(Erc20Error::ZeroRecipientAddress);
        }

        self._before_token_transfer(from, to, amount)?;

        let from_balance = self._balance_of(from);
        if from_balance < amount {
            return Err(Erc20Error::InsufficientBalance);
        }
        self._balances_mut().insert(from, from_balance - amount);
        let to_balance = self._balance_of(to);
        self._balances_mut().insert(to, to_balance + amount);
        self._emit_transfer_event(Some(from), Some(to), amount);
        Ok(())
    }

    fn _approve_from_to(&mut self, owner: AccountId, spender: AccountId, amount: Balance) -> Result<(), Erc20Error> {
        if owner == ZERO_ADDRESS.into() {
            return Err(Erc20Error::ZeroSenderAddress);
        }
        if spender == ZERO_ADDRESS.into() {
            return Err(Erc20Error::ZeroRecipientAddress);
        }

        self._allowances_mut().insert((owner, spender), amount);
        self._emit_approval_event(owner, spender, amount);
        Ok(())
    }
}

pub trait Erc20: Erc20Internal {
    /// Returns the token name.
    fn _token_name(&self) -> Option<String> {
        Lazy::get(self._name()).clone()
    }

    /// Returns the token symbol.
    fn _token_symbol(&self) -> Option<String> {
        Lazy::get(self._symbol()).clone()
    }

    /// Returns the token decimals.
    fn _token_decimals(&self) -> u8 {
        Lazy::get(self._decimals()).clone()
    }

    /// Returns the total token supply.
    fn _total_supply(&self) -> Balance {
        Lazy::get(self._supply()).clone()
    }

    /// Returns the account Balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    // This function is implemented in Erc20Internal
    // fn _balance_of(&self, owner: AccountId) -> Balance;

    /// Transfers `value` amount of tokens from the caller's account to account `to`.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the caller's account Balance.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
        let from = Self::env().caller();
        self._transfer_from_to(from, to, value)
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set `0`.
    fn _allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self._allowances().get(&(owner, spender)).copied().unwrap_or(0)
    }

    /// Transfers `value` tokens on the behalf of `from` to the account `to`.
    ///
    /// This can be used to allow a contract to transfer tokens on ones behalf and/or
    /// to charge fees in sub-currencies, for example.
    ///
    /// On success a `Transfer` and `Approval` events are emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `from`.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
        let caller = Self::env().caller();
        let allowance = self._allowance(from, caller);
        if allowance < value {
            return Err(Erc20Error::InsufficientAllowance);
        }
        self._transfer_from_to(from, to, value)?;
        self._approve_from_to(from, caller, allowance - value)?;
        Ok(())
    }

    /// Allows `spender` to withdraw from the caller's account multiple times, up to
    /// the `value` amount.
    ///
    /// If this function is called again it overwrites the current allowance with `value`.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, value)
    }

    /// Sets the decimals
    fn _set_decimals(&mut self, decimals: u8) {
        *self._decimals_mut() = Lazy::new(decimals);
    }

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
        let owner = Self::env().caller();
        self._approve_from_to(owner, spender, self._allowance(owner, spender) + delta_value)
    }

    /// Atomically decreases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
        let owner = Self::env().caller();
        let allowance = self._allowance(owner, spender);
        if allowance < delta_value {
            return Err(Erc20Error::InsufficientAllowance);
        }

        self._approve_from_to(owner, spender, allowance - delta_value)
    }

    /// Creates `amount` tokens and assigns them to `account`, increasing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), Erc20Error> {
        if account == ZERO_ADDRESS.into() {
            return Err(Erc20Error::ZeroRecipientAddress);
        }

        let mut new_balance = self._balance_of(account);
        new_balance += amount;
        self._balances_mut().insert(account, new_balance);
        *self._supply_mut() = Lazy::new(self._total_supply() + amount);
        self._emit_transfer_event(None, Some(account), amount);
        Ok(())
    }

    /// Destroys `amount` tokens from `account`, reducing the total supply.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if recipient's address is zero.
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `account`.
    fn _burn(&mut self, account: AccountId, amount: Balance) -> Result<(), Erc20Error> {
        if account == ZERO_ADDRESS.into() {
            return Err(Erc20Error::ZeroSenderAddress);
        }

        let mut from_balance = self._balance_of(account);
        if from_balance < amount {
            return Err(Erc20Error::InsufficientBalance);
        }

        from_balance -= amount;
        self._balances_mut().insert(account, from_balance);
        *self._supply_mut() = Lazy::new(self._total_supply() - amount);
        self._emit_transfer_event(Some(account), None, amount);
        Ok(())
    }
}

#[cfg(test)]
#[ink_lang::contract]
mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink_lang as ink;
    use utils::{
        traits::{InkStorage},
        iml_getters,
    };
    use ink::{Env, EmitEvent};
    use crate::traits::{ IErc20 };
    use ink_env::{
        hash::{
            Blake2x256,
            CryptoHash,
            HashOutput,
        },
        Clear,
    };

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
    impl Erc20Internal for Erc20Struct {
        fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                spender: _spender,
                value: _amount,
            });
        }
    }
    impl Erc20 for Erc20Struct {}
    impl IErc20 for Erc20Struct {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self._token_name()
        }

        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self._token_symbol()
        }

        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self._token_decimals()
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self._total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self._balance_of(owner)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            self._transfer(to, value)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self._allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            self._transfer_from(from, to, value)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
            self._approve(spender, value)
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
            self._increase_allowance(spender, delta_value)
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
            self._decrease_allowance(spender, delta_value)
        }
    }
    
    impl Erc20Struct {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance) -> Self {
            let mut instance = Self::_empty();
            instance._mint(instance.env().caller(), _total_supply).expect("Can't mint tokens");
            instance
        }
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
        assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
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
        assert_eq!(
            erc20.transfer(accounts.eve, 10),
            Err(Erc20Error::InsufficientBalance)
        );
        // Alice owns all the tokens.
        assert_eq!(erc20.balance_of(accounts.alice), 100);
        assert_eq!(erc20.balance_of(accounts.bob), 0);
        assert_eq!(erc20.balance_of(accounts.eve), 0);

        // Transfer event triggered during initial construction.
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events.len(), 1);
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(AccountId::from([0x01; 32])),
            100,
        );
    }

    #[ink::test]
    fn transfer_from_works() {
        // Constructor works.
        let mut erc20 = Erc20Struct::new(100);
        // Transfer event triggered during initial construction.
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        // Bob fails to transfer tokens owned by Alice.
        assert_eq!(
            erc20.transfer_from(accounts.alice, accounts.eve, 10),
            Err(Erc20Error::InsufficientAllowance)
        );
        // Alice approves Bob for token transfers on her behalf.
        assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

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
        assert_eq!(
            erc20.transfer_from(accounts.alice, accounts.eve, 10),
            Ok(())
        );
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
    fn allowance_must_not_change_on_failed_transfer() {
        let mut erc20 = Erc20Struct::new(100);
        let accounts =
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

        // Alice approves Bob for token transfers on her behalf.
        let alice_balance = erc20.balance_of(accounts.alice);
        let initial_allowance = alice_balance + 2;
        assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

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

        // Bob tries to transfer tokens from Alice to Eve.
        let emitted_events_before =
            ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(
            erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
            Err(Erc20Error::InsufficientBalance)
        );
        // Allowance must have stayed the same
        assert_eq!(
            erc20.allowance(accounts.alice, accounts.bob),
            initial_allowance
        );
        // No more events must have been emitted
        let emitted_events_after =
            ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events_before.len(), emitted_events_after.len());
    }
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