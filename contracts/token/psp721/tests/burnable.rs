#[cfg(test)]
#[brush::contract]
mod burnable {
    use brush::traits::ZERO_ADDRESS;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_env::{
        call,
        test,
    };
    use ink_lang as ink;
    use ink_prelude::string::String;
    use psp721::{
        extensions::burnable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
    }

    impl PSP721 for PSP721Struct {}

    impl PSP721Burnable for PSP721Struct {}

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._mint([1; 32]);
            instance
        }
    }

    #[ink::test]
    fn burn_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Destroy token Id 1.
        nft.burn([1; 32]);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of([1; 32]), None);
    }

    #[ink::test]
    #[should_panic(expected = "TokenNotFound")]
    fn burn_fails_token_not_found() {
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Try burning a non existent token
        nft.burn([4; 32]);
    }

    #[ink::test]
    #[should_panic(expected = "NotOwner")]
    fn burn_fails_not_owner() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Try burning this token with a different account
        change_callee(accounts.eve);
        nft.burn([1; 32]);
    }

    fn change_callee(account: AccountId) {
        // CHANGE CALLEE MANUALLY
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
        // Create call.
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&account);
        // Push the new execution context to set Bob as caller.
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(account, callee, 1000000, 1000000, data);
    }
}
