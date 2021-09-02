#[cfg(test)]
#[brush::contract]
mod tests {
    use payment_splitter::traits::*;
    use ::ink_env::DefaultEnvironment;
    use ink_env::test::DefaultAccounts;
    use ink_lang as ink;

    use ink::{
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
    #[derive(Default, PaymentSplitterStorage)]
    pub struct MySplitter {
        #[PaymentSplitterStorageField]
        splitter: PaymentSplitterData,
    }

    impl MySplitter {
        #[ink(constructor)]
        pub fn new(payees: Vec<AccountId>, shares: Vec<Balance>) -> Self {
            let mut instance = Self::default();
            instance._init(payees, shares);
            instance
        }
    }

    impl PaymentSplitter for MySplitter {
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

    type Event = <MySplitter as ::ink_lang::BaseEvent>::Type;

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        accounts
    }

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
        let accounts = setup();
        let instance = MySplitter::new(vec![accounts.alice, accounts.bob], vec![100, 200]);

        assert_eq!(100 + 200, instance.total_shares());
        assert_eq!(0, instance.total_released());
        assert_eq!(accounts.alice, instance.payee(0));
        assert_eq!(accounts.bob, instance.payee(1));

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payee_added_event(&emitted_events[0], accounts.alice, 100);
        assert_payee_added_event(&emitted_events[1], accounts.bob, 200);
    }

    #[ink::test]
    #[should_panic(expected = "LengthMismatch")]
    fn fails_init() {
        let accounts = setup();
        let _ = MySplitter::new(vec![accounts.alice, accounts.bob], vec![100]);
    }

    #[ink::test]
    fn correct_release() {
        let accounts = setup();
        let mut instance = MySplitter::new(vec![accounts.alice, accounts.bob], vec![100, 200]);
        assert!(ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.alice, 0).is_ok());
        assert!(ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.bob, 0).is_ok());
        let amount = 1000000;
        add_funds(instance.env().account_id(), amount);

        assert_eq!(100 + 200, instance.total_shares());
        instance.release(accounts.alice);
        assert_eq!(333314, instance.total_released());
        instance.release(accounts.bob);
        assert_eq!(999942, instance.total_released());
        assert_eq!(333314, instance.released(accounts.alice));
        assert_eq!(
            333314,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.alice).unwrap()
        );
        assert_eq!(2 * 333314, instance.released(accounts.bob));
        assert_eq!(
            2 * 333314,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.bob).unwrap()
        );

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payment_released_event(&emitted_events[2], accounts.alice, 333314);
        assert_payment_released_event(&emitted_events[3], accounts.bob, 2 * 333314);
    }

    #[ink::test]
    fn correct_second_release() {
        let accounts = setup();
        let mut instance = MySplitter::new(vec![accounts.alice, accounts.bob], vec![100, 200]);
        let amount = 1000000;
        add_funds(instance.env().account_id(), amount);
        instance.release(accounts.alice);
        instance.release(accounts.bob);

        assert!(ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.alice, 0).is_ok());
        assert!(ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(accounts.bob, 0).is_ok());

        add_funds(instance.env().account_id(), amount);
        instance.release(accounts.alice);
        instance.release(accounts.bob);
        assert_eq!(1999884, instance.total_released());
        assert_eq!(666628, instance.released(accounts.alice));
        assert_eq!(
            333314,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.alice).unwrap()
        );
        assert_eq!(1333256, instance.released(accounts.bob));
        assert_eq!(
            666628,
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(accounts.bob).unwrap()
        );

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_payment_released_event(&emitted_events[2], accounts.alice, 333314);
        assert_payment_released_event(&emitted_events[3], accounts.bob, 666628);
        assert_payment_released_event(&emitted_events[4], accounts.alice, 333314);
        assert_payment_released_event(&emitted_events[5], accounts.bob, 666628);
    }

    #[ink::test]
    #[should_panic(expected = "AccountIsNotDuePayment")]
    fn correct_release_with_zero_payment() {
        let accounts = setup();
        let mut instance = MySplitter::new(vec![accounts.alice, accounts.bob], vec![100, 200]);

        instance.release(accounts.alice);
    }

    #[ink::test]
    #[should_panic(expected = "AccountHasNoShares")]
    fn correct_release_unknown_account() {
        let accounts = setup();
        let mut instance = MySplitter::new(vec![accounts.alice, accounts.bob], vec![100, 200]);

        instance.release(accounts.eve);
    }

    fn add_funds(account: AccountId, amount: Balance) {
        assert!(ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account, amount,).is_ok());
    }
}
