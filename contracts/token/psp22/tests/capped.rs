#[brush::contract]
mod capped {
    use brush::test_utils::accounts;
    use ink_lang as ink;
    use psp22::{
        extensions::capped::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22CappedStorage)]
    pub struct PSP22CappedStruct {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22CappedStorageField]
        capped: PSP22CappedData,
    }

    impl PSP22 for PSP22CappedStruct {}

    impl PSP22Capped for PSP22CappedStruct {}

    impl PSP22CappedStruct {
        #[ink(constructor)]
        pub fn new(inital_supply: Balance, cap: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance.init_cap(cap).is_ok());
            assert!(instance._mint(instance.env().caller(), inital_supply).is_ok());
            instance
        }

        fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            if (self.total_supply() + amount) > self.cap() {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")))
            }
            PSP22::_mint(self, account, amount)
        }
    }

    #[ink::test]
    fn new_works() {
        let accounts = accounts();
        let mint_amount = 1000;
        let cap = 2000;
        let capped = PSP22CappedStruct::new(mint_amount, cap);

        assert_eq!(capped.balance_of(accounts.alice), mint_amount);
        assert_eq!(capped.total_supply(), mint_amount);
        assert_eq!(capped.cap(), cap);
    }
}
