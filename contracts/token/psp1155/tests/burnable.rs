#[cfg(test)]
#[brush::contract]
mod burnable {
    use ink_lang as ink;
    use psp1155::{
        extensions::burnable::*,
        traits::*,
    };

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
    }

    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Burnable for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            instance._mint(accounts.alice, [1; 32], 20);
            instance._mint(accounts.bob, [1; 32], 20);
            instance._mint(accounts.bob, [2; 32], 20);
            instance
        }
    }

    #[ink::test]
    fn burn_works() {
        let token_id_1 = [1; 32];
        let token_1_amount = 20;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();

        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id_1), token_1_amount);
        // switch to bob, give allowance to alice so we can burn_from bob by alice
        change_callee(accounts.bob);
        nft.set_approval_for_all(accounts.alice, true);
        change_callee(accounts.alice);

        nft.burn(token_id_1, token_1_amount);
        nft.burn_from(accounts.bob, token_id_1, token_1_amount);
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id_1), 0);
    }

    #[ink::test]
    fn burn_batch_works() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let burn_ids = vec![token_id_1, token_id_2];
        let burn_amounts = vec![1, 1];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();

        nft.burn_batch(
            burn_ids
                .clone()
                .iter()
                .zip(burn_amounts.clone().iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
        let balance_1 = nft.balance_of(accounts.alice, token_id_1);
        let balance_2 = nft.balance_of(accounts.alice, token_id_2);

        nft.burn_batch(
            burn_ids
                .clone()
                .iter()
                .zip(burn_amounts.clone().iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
        let balance_3 = nft.balance_of(accounts.alice, token_id_1);
        let balance_4 = nft.balance_of(accounts.alice, token_id_2);

        // switch to bob, give allowance to alice so we can burn_from bob by alice
        change_callee(accounts.bob);
        nft.set_approval_for_all(accounts.alice, true);
        change_callee(accounts.alice);

        nft.burn_batch_from(
            accounts.bob,
            burn_ids
                .clone()
                .iter()
                .zip(burn_amounts.clone().iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
        let balance_5 = nft.balance_of(accounts.bob, token_id_1);
        let balance_6 = nft.balance_of(accounts.bob, token_id_2);

        nft.burn_batch_from(
            accounts.bob,
            burn_ids
                .iter()
                .zip(burn_amounts.iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
        let balance_7 = nft.balance_of(accounts.bob, token_id_1);
        let balance_8 = nft.balance_of(accounts.bob, token_id_2);

        assert_eq!(balance_1, 1);
        assert_eq!(balance_2, 1);
        assert_eq!(balance_3, 0);
        assert_eq!(balance_4, 0);
        assert_eq!(balance_5, 1);
        assert_eq!(balance_6, 1);
        assert_eq!(balance_7, 0);
        assert_eq!(balance_8, 0);
    }

    #[ink::test]
    #[should_panic(expected = "NotAllowed")]
    fn burn_from_without_allowance() {
        let token_id_1 = [1; 32];
        let token_1_amount = 20;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();

        nft.burn_from(accounts.bob, token_id_1, token_1_amount);
    }

    #[ink::test]
    #[should_panic(expected = "NotAllowed")]
    fn burn_batch_from_without_allowance() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_amount = 1;
        let burn_ids = vec![token_id_1, token_id_2];
        let burn_amounts = vec![token_amount, token_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();

        nft.burn_batch_from(
            accounts.bob,
            burn_ids
                .iter()
                .zip(burn_amounts.iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn burn_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 2;

        let mut nft = PSP1155Struct::new();

        nft.burn(token_id_1, burn_amount);
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn burn_from_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 21;
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();

        // switch to bob, give allowance to alice so we can burn_from bob by alice
        change_callee(accounts.bob);
        nft.set_approval_for_all(accounts.alice, true);
        change_callee(accounts.alice);

        nft.burn_from(accounts.bob, token_id_1, burn_amount);
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn burn_batch_insufficient_balance() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_amount = 2;
        let burn_ids = vec![token_id_1, token_id_2];
        let burn_amounts = vec![token_amount, token_amount];

        let mut nft = PSP1155Struct::new();

        nft.burn_batch(
            burn_ids
                .iter()
                .zip(burn_amounts.iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
    }

    #[ink::test]
    #[should_panic(expected = "InsufficientBalance")]
    fn burn_batch_from_insufficient_balance() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_amount = 2;
        let burn_ids = vec![token_id_1, token_id_2];
        let burn_amounts = vec![token_amount, token_amount];
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();

        // switch to bob, give allowance to alice so we can burn_from bob by alice
        change_callee(accounts.bob);
        nft.set_approval_for_all(accounts.alice, true);
        change_callee(accounts.alice);

        nft.burn_batch_from(
            accounts.bob,
            burn_ids
                .iter()
                .zip(burn_amounts.iter())
                .map(|(id, amount)| (id.clone(), amount.clone()))
                .collect(),
        );
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
