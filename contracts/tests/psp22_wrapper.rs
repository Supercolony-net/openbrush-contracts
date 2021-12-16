#![feature(min_specialization)]
#[cfg(feature = "psp22")]
#[brush::contract]
mod psp22_wrapper {
    use contracts::psp22::extensions::wrapper::*;
    use ink_lang as ink;

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22WrapperStorage)]
    pub struct PSP22WrapperStruct {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22WrapperStorageField]
        wrapper: PSP22WrapperData,
        contract_balance: Balance,
    }

    impl PSP22 for PSP22WrapperStruct {}

    /// We will override cross-contract wrapper calls in tests
    /// The cross-contract interaction will be tested in integration tests
    impl PSP22WrapperInternal for PSP22WrapperStruct {
        fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            self.contract_balance += amount;
            Ok(())
        }

        fn _withdraw(&mut self, _account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self.contract_balance -= amount;
            Ok(())
        }

        fn _underlying_balance(&mut self) -> Balance {
            self.contract_balance
        }
    }

    impl PSP22Wrapper for PSP22WrapperStruct {}

    impl PSP22WrapperStruct {
        #[ink(constructor)]
        pub fn new(underlying: AccountId) -> Self {
            let mut instance = Self::default();
            instance._init(underlying);
            instance
        }

        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
            self._recover(self.env().caller())
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            self._burn(self.env().caller(), amount)
        }
    }

    #[ink::test]
    fn deposit_for_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert_eq!(wrapper.balance_of(accounts.alice), 0);
        assert_eq!(wrapper.total_supply(), 0);

        assert!(wrapper.deposit_for(accounts.alice, 100).is_ok());

        assert_eq!(wrapper.balance_of(accounts.alice), 100);
        assert_eq!(wrapper.total_supply(), 100);
    }

    #[ink::test]
    fn withdraw_to_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert!(wrapper.deposit_for(accounts.alice, 100).is_ok());
        assert_eq!(wrapper.balance_of(accounts.alice), 100);
        assert_eq!(wrapper.total_supply(), 100);
        assert!(wrapper.withdraw_to(accounts.alice, 100).is_ok());

        assert_eq!(wrapper.balance_of(accounts.alice), 0);
        assert_eq!(wrapper.total_supply(), 0);
    }

    #[ink::test]
    fn recover_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert!(wrapper.deposit_for(accounts.alice, 100).is_ok());
        assert!(wrapper.burn(100).is_ok());
        assert_eq!(wrapper.balance_of(accounts.alice), 0);
        assert_eq!(wrapper.total_supply(), 0);

        assert!(wrapper.recover().is_ok());

        assert_eq!(wrapper.balance_of(accounts.alice), 100);
        assert_eq!(wrapper.total_supply(), 100);
    }
}
