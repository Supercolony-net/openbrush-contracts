#[cfg(test)]
#[brush::contract]
mod tests {
    use ::ink_env::DefaultEnvironment;
    use brush::test_utils::accounts;
    use ink_env::test::DefaultAccounts;
    use ink_lang as ink;
    use pausable::traits::*;

    use ink::{
        EmitEvent,
        Env,
    };

    /// Emitted when the pause is triggered by `account`.
    #[ink(event)]
    pub struct Paused {
        pub account: AccountId,
    }

    /// Emitted when the pause is lifted by `account`.
    #[ink(event)]
    pub struct Unpaused {
        pub account: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, PausableStorage)]
    pub struct MyFlipper {
        #[PausableStorageField]
        pause: PausableData,
        flipped: bool,
    }

    impl MyFlipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        #[brush::modifiers(when_paused)]
        pub fn flip(&mut self) -> Result<bool, PausableError> {
            let previous = self.flipped;
            self.flipped = !previous;

            Ok(previous)
        }
    }

    impl Pausable for MyFlipper {
        fn _emit_paused_event(&self, account: AccountId) {
            self.env().emit_event(Paused { account })
        }

        fn _emit_unpaused_event(&self, account: AccountId) {
            self.env().emit_event(Unpaused { account })
        }
    }

    type Event = <MyFlipper as ::ink_lang::BaseEvent>::Type;

    fn assert_paused_event(event: &ink_env::test::EmittedEvent, expected_account: AccountId) {
        if let Event::Paused(Paused { account }) = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                account, expected_account,
                "Accounts were not equal: encountered {:?}, expected {:?}",
                account, expected_account
            );
        }
    }

    fn assert_unpaused_event(event: &ink_env::test::EmittedEvent, expected_account: AccountId) {
        if let Event::Unpaused(Unpaused { account }) = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer")
        {
            assert_eq!(
                account, expected_account,
                "Accounts were not equal: encountered {:?}, expected {:?}",
                account, expected_account
            );
        }
    }

    fn setup() -> DefaultAccounts<DefaultEnvironment> {
        let accounts = accounts();

        accounts
    }

    #[ink::test]
    fn pause_works() {
        let accounts = setup();
        let mut inst = MyFlipper::new();
        assert!(inst._pause::<PausableError>().is_ok());
        assert!(inst.pause.paused);

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_paused_event(&emitted_events[0], accounts.alice);
    }

    #[ink::test]
    fn double_pause_fails() {
        let mut inst = MyFlipper::new();
        assert!(inst._pause::<PausableError>().is_ok());
        assert_eq!(Err(PausableError::Paused), inst._pause());
    }

    #[ink::test]
    fn flip_works() {
        let mut inst = MyFlipper::new();
        assert!(inst._pause::<PausableError>().is_ok());

        assert_eq!(Ok(false), inst.flip());
        assert_eq!(Ok(true), inst.flip());
        assert_eq!(Ok(false), inst.flip());
    }

    #[ink::test]
    fn flip_fails() {
        let mut inst = MyFlipper::new();

        assert_eq!(Err(PausableError::NoPaused), inst.flip());
    }

    #[ink::test]
    fn unpause_fails() {
        let mut inst = MyFlipper::new();

        assert_eq!(Err(PausableError::NoPaused), inst._unpause());
    }

    #[ink::test]
    fn unpause_works() {
        let accounts = setup();
        let mut inst = MyFlipper::new();

        assert!(inst._pause::<PausableError>().is_ok());
        assert!(inst._unpause::<PausableError>().is_ok());
        assert!(!inst.pause.paused);

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_unpaused_event(&emitted_events[0], accounts.alice);
    }
}
