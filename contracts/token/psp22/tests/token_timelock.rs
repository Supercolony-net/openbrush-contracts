#[brush::contract]
mod tests {
    use brush::traits::InkStorage;
    use ink_lang as ink;
    use psp22::{
        traits::*,
        utils::token_timelock::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22TokenTimelockStorage)]
    pub struct PSP22TokenTimelockStruct {
        #[PSP22TokenTimelockStorageField]
        timelock: PSP22TokenTimelockData,
        locked_tokens: Balance,
    }

    /// We will override timelock functions so they are not using cross-contract call in tests
    /// We will just remove calls to the locked token
    /// The cross-contract interaction will be tested in integration tests
    impl PSP22TokenTimelock for PSP22TokenTimelockStruct {
        fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            self.locked_tokens -= amount;
            Ok(())
        }

        fn contract_balance(&self) -> Balance {
            self.locked_tokens
        }
    }

    impl PSP22TokenTimelockStruct {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            let mut instance = Self::default();
            instance.init(token_address, beneficiary, release_time);
            instance
        }

        /// Helper function for mocked deposit
        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            self.locked_tokens += amount;
        }

        /// Helper function to get mocked balance
        #[ink(message)]
        pub fn balance(&mut self) -> Balance {
            self.locked_tokens
        }
    }

    /// Test the constructor
    #[ink::test]
    fn new_works() {
        let accounts = get_accounts();
        let timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());

        assert_eq!(timelock.token(), AccountId::from([0x1; 32]));
        assert_eq!(timelock.beneficiary(), accounts.alice);
        assert_eq!(timelock.release_time(), day());
    }

    #[ink::test]
    fn release_works() {
        let deposited_tokens = 1000;
        let accounts = get_accounts();
        let mut timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());
        // deposit tokens to the contract
        timelock.deposit(deposited_tokens);
        // pass one day
        for _ in 0..day_blocks() {
            advance_block();
        }
        assert_eq!(timelock.balance(), deposited_tokens);

        // release the tokens
        timelock.release();

        // timelock should be empty
        assert_eq!(timelock.balance(), 0);
    }

    #[ink::test]
    #[should_panic(expected = "Custom")]
    fn release_soon_should_panic() {
        let deposited_tokens = 1000;
        let accounts = get_accounts();
        let mut timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());
        // deposit tokens to the contract
        timelock.deposit(deposited_tokens);

        // release the tokens should panic
        timelock.release();
    }

    #[ink::test]
    #[should_panic(expected = "Custom")]
    fn release_without_deposit_should_panic() {
        let accounts = get_accounts();
        let mut timelock = PSP22TokenTimelockStruct::new(AccountId::from([0x1; 32]), accounts.alice, day());
        // pass one day
        for _ in 0..day_blocks() {
            advance_block();
        }
        assert_eq!(timelock.balance(), 0);

        // release the tokens
        timelock.release();
    }

    type DefEnv = ink_env::DefaultEnvironment;

    fn day_blocks() -> u32 {
        (60 * 60 * 24) / 5
    }

    fn day() -> Timestamp {
        get_time() + (60 * 60 * 24)
    }

    fn advance_block() {
        ink_env::test::advance_block::<DefEnv>().expect("Cannot advance block");
    }

    fn get_accounts() -> ink_env::test::DefaultAccounts<DefEnv> {
        ink_env::test::default_accounts::<DefEnv>().expect("Cannot get accounts")
    }

    fn get_time() -> Timestamp {
        ink_env::block_timestamp::<DefEnv>().expect("Cannot get block timestamp")
    }
}
