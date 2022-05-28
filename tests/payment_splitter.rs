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
#[cfg(feature = "payment_splitter")]
#[openbrush::contract]
mod payment_splitter {
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::payment_splitter::*,
        test_utils::accounts,
    };

    use ink::codegen::{
        EmitEvent,
        Env,
    };

    #[ink(event)]
    pub struct PayeeAdded {
        pub account: AccountId,
        pub shares: Balance,
    }

    #[ink(event)]
    pub struct PaymentReceived {
        pub from: AccountId,
        pub amount: Balance,
    }

    #[ink(event)]
    pub struct PaymentReleased {
        pub to: AccountId,
        pub amount: Balance,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PaymentSplitterStorage)]
    pub struct MySplitter {
        #[PaymentSplitterStorageField]
        splitter: PaymentSplitterData,
    }

    impl MySplitter {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut MySplitter| {
                instance._init(payees_and_shares).unwrap();
            })
        }
    }

    impl PaymentSplitter for MySplitter {}

    impl PaymentSplitterInternal for MySplitter {
        fn _emit_payee_added_event(&self, account: AccountId, shares: Balance) {
            self.env().emit_event(PayeeAdded { account, shares })
        }

        fn _emit_payment_received_event(&self, from: AccountId, amount: Balance) {
            self.env().emit_event(PaymentReceived { from, amount })
        }

        fn _emit_payment_released_event(&self, to: AccountId, amount: Balance) {
            self.env().emit_event(PaymentReleased { to, amount })
        }
    }

    type Event = <MySplitter as ::ink_lang::reflect::ContractEventBase>::Type;

    fn assert_payee_added_event(
        event: &ink_env::test::EmittedEvent,
        expected_account: AccountId,
        expected_shares: Balance,
    ) {
        if let Event::PayeeAdded(PayeeAdded { account, shares }) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                account, expected_account,
                "Accounts were not equal: encountered {:?}, expected {:?}",
                account, expected_account
            );
            assert_eq!(
                shares, expected_shares,
                "Shares were not equal: encountered {:?}, expected {:?}",
                shares, expected_shares
            );
        }
    }

    fn assert_payment_released_event(
        event: &ink_env::test::EmittedEvent,
        expected_to: AccountId,
        expected_amount: Balance,
    ) {
        if let Event::PaymentReleased(PaymentReleased { to, amount }) =
            <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                to, expected_to,
                "Accounts were not equal: encountered {:?}, expected {:?}",
                to, expected_to
            );
            assert_eq!(
                amount, expected_amount,
                "Amounts were not equal: encountered {:?}, expected {:?}",
                amount, expected_amount
            );
        }
    }

    #[ink::test]
    fn correct_init_values() {
        let accounts = accounts();
        let instance = MySplitter::new(vec![(accounts.alice, 100), (accounts.bob, 200)]);

        assert_eq!(100 + 200, instance.total_shares());
        assert_eq!(0, instance.total_released());
        assert_eq!(accounts.alice, instance.payee(0));
        assert_eq!(accounts.bob, instance.payee(1));

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payee_added_event(&emitted_events[0], accounts.alice, 100);
        assert_payee_added_event(&emitted_events[1], accounts.bob, 200);
    }

    #[ink::test]
    fn correct_release() {
        let accounts = accounts();
        let mut instance = MySplitter::new(vec![(accounts.charlie, 100), (accounts.bob, 200)]);
        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.charlie, 0);
        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.bob, 0);
        let amount = 1000000;
        add_funds(instance.env().account_id(), amount);

        assert_eq!(100 + 200, instance.total_shares());
        assert!(instance.release(accounts.charlie).is_ok());
        assert_eq!(333333, instance.total_released());
        assert!(instance.release(accounts.bob).is_ok());
        assert_eq!(999999, instance.total_released());
        assert_eq!(333333, instance.released(accounts.charlie));
        assert_eq!(
            333333,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.charlie).unwrap()
        );
        assert_eq!(2 * 333333, instance.released(accounts.bob));
        assert_eq!(
            2 * 333333,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.bob).unwrap()
        );

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payment_released_event(&emitted_events[2], accounts.charlie, 333333);
        assert_payment_released_event(&emitted_events[3], accounts.bob, 2 * 333333);
    }

    #[ink::test]
    fn correct_second_release() {
        let accounts = accounts();
        let mut instance = MySplitter::new(vec![(accounts.charlie, 100), (accounts.bob, 200)]);
        let amount = 1000000;
        add_funds(instance.env().account_id(), amount);
        assert!(instance.release(accounts.charlie).is_ok());
        assert!(instance.release(accounts.bob).is_ok());

        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.charlie, 0);
        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.bob, 0);

        add_funds(instance.env().account_id(), amount);
        assert!(instance.release(accounts.charlie).is_ok());
        assert!(instance.release(accounts.bob).is_ok());
        assert_eq!(1999999, instance.total_released());
        assert_eq!(666666, instance.released(accounts.charlie));
        assert_eq!(
            333333,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.charlie).unwrap()
        );
        assert_eq!(1333333, instance.released(accounts.bob));
        assert_eq!(
            666667,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.bob).unwrap()
        );

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payment_released_event(&emitted_events[2], accounts.charlie, 333333);
        assert_payment_released_event(&emitted_events[3], accounts.bob, 666666);
        assert_payment_released_event(&emitted_events[4], accounts.charlie, 333333);
        assert_payment_released_event(&emitted_events[5], accounts.bob, 666667);
    }

    #[ink::test]
    fn correct_release_with_zero_payment() {
        let accounts = accounts();
        let mut instance = MySplitter::new(vec![(accounts.alice, 101), (accounts.bob, 200)]);

        assert_eq!(
            Err(PaymentSplitterError::AccountIsNotDuePayment),
            instance.release(accounts.alice)
        );
    }

    #[ink::test]
    fn correct_release_unknown_account() {
        let accounts = accounts();
        let mut instance = MySplitter::new(vec![(accounts.alice, 100), (accounts.bob, 200)]);

        assert_eq!(
            Err(PaymentSplitterError::AccountHasNoShares),
            instance.release(accounts.eve)
        );
    }

    #[ink::test]
    fn correct_release_all() {
        let accounts = accounts();
        let mut instance = MySplitter::new(vec![(accounts.charlie, 100), (accounts.bob, 200)]);
        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.charlie, 0);
        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.bob, 0);
        let amount = 1000000;
        add_funds(instance.env().account_id(), amount);

        assert_eq!(100 + 200, instance.total_shares());
        assert!(instance._release_all().is_ok());
        assert_eq!(999999, instance.total_released());
        assert_eq!(333333, instance.released(accounts.charlie));
        assert_eq!(
            333333,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.charlie).unwrap()
        );
        assert_eq!(2 * 333333, instance.released(accounts.bob));
        assert_eq!(
            2 * 333333,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.bob).unwrap()
        );

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payment_released_event(&emitted_events[2], accounts.charlie, 333333);
        assert_payment_released_event(&emitted_events[3], accounts.bob, 2 * 333333);
    }

    fn add_funds(account: AccountId, amount: Balance) {
        let balance = ink_env::balance::<ink_env::DefaultEnvironment>();
        ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account, balance + amount);
    }
}
