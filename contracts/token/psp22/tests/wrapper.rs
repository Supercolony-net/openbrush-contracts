#[brush::contract]
mod tests {
    use brush::traits::InkStorage;
    use ink_lang as ink;
    use psp22::{
        extensions::wrapper::*,
        traits::*,
    };

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

    /// We will override wrapper functions so they are not using cross-contract call in tests
    /// We will just remove calls to transfer of underlying token
    /// The cross-contract interaction will be tested in integration tests
    impl PSP22Wrapper for PSP22WrapperStruct {
        #[ink(message)]
        fn deposit_for(&mut self, account: AccountId, amount: Balance) -> bool {
            self.contract_balance += amount;
            self._mint(account, amount);
            true
        }

        #[ink(message)]
        fn withdraw_to(&mut self, _account: AccountId, amount: Balance) -> bool {
            self.contract_balance -= amount;
            self._burn(<PSP22WrapperStruct as InkStorage>::env().caller(), amount);
            true
        }

        fn _recover(&mut self, account: AccountId) -> Balance {
            let value = self.contract_balance - self.total_supply();
            self._mint(account, value);
            value
        }
    }

    impl PSP22Receiver for PSP22WrapperStruct {
        #[ink(message)]
        fn before_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22ReceiverError> {
            Ok(())
        }
    }

    impl PSP22WrapperStruct {
        #[ink(constructor)]
        pub fn new(underlying: AccountId) -> Self {
            let mut instance = Self::default();
            instance.init(underlying);
            instance
        }

        #[ink(message)]
        pub fn underlying(&mut self) -> AccountId {
            self.wrapper.underlying
        }

        #[ink(message)]
        pub fn recover(&mut self) -> Balance {
            self._recover(<PSP22WrapperStruct as InkStorage>::env().caller())
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: Balance){
            self._burn(<PSP22WrapperStruct as InkStorage>::env().caller(), amount);
        }
    }

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        let underlying = AccountId::from([0x1; 32]);
        let mut wrapper = PSP22WrapperStruct::new(underlying);

        assert_eq!(wrapper.underlying(), underlying);
    }

    #[ink::test]
    fn deposit_for_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        assert_eq!(wrapper.balance_of(accounts.alice), 0);
        assert_eq!(wrapper.total_supply(), 0);

        wrapper.deposit_for(accounts.alice, 100);

        assert_eq!(wrapper.balance_of(accounts.alice), 100);
        assert_eq!(wrapper.total_supply(), 100);
    }

    #[ink::test]
    fn withdraw_to_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        wrapper.deposit_for(accounts.alice, 100);
        assert_eq!(wrapper.balance_of(accounts.alice), 100);
        assert_eq!(wrapper.total_supply(), 100);
        
        wrapper.withdraw_to(accounts.alice, 100);

        assert_eq!(wrapper.balance_of(accounts.alice), 0);
        assert_eq!(wrapper.total_supply(), 0);
    }

    #[ink::test]
    fn recover_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        let mut wrapper = PSP22WrapperStruct::new(AccountId::from([0x1; 32]));

        wrapper.deposit_for(accounts.alice, 100);
        wrapper.burn(100);
        assert_eq!(wrapper.balance_of(accounts.alice), 0);
        assert_eq!(wrapper.total_supply(), 0);

        wrapper.recover();

        assert_eq!(wrapper.balance_of(accounts.alice), 100);
        assert_eq!(wrapper.total_supply(), 100);
    }
}
