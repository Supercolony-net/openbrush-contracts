#[cfg(test)]
#[brush::contract]
mod metadata {
    use brush::test_utils::accounts;
    use ink_lang as ink;
    use psp721::{
        extensions::enumerable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage, PSP721EnumerableStorage)]
    #[ink(storage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[PSP721EnumerableStorageField]
        enumerable: PSP721EnumerableData,
    }

    impl PSP721Enumerable for PSP721Struct {}

    impl PSP721 for PSP721Struct {
        /// Here we will override `_before_token_transfer` to use check
        /// for checks in `handle_token_transfer` function
        fn _before_token_transfer(&mut self, from: &AccountId, to: &AccountId, id: &Id) -> Result<(), PSP721Error> {
            // call super `_before_token_transfer` here
            self._handle_token_transfer(from, to, id);
            Ok(())
        }

        fn _do_safe_transfer_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _data: Vec<u8>,
        ) -> Result<(), PSP721Error> {
            Ok(())
        }
    }

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[ink::test]
    fn mint_first_token() {
        let accounts = accounts();
        let mut nft = PSP721Struct::new();
        let token_id = [0x1; 32];

        assert_eq!(nft.total_supply(), 0);
        assert_eq!(nft.balance_of(accounts.alice), 0);

        // mint first token for alice
        assert!(nft._mint_to(accounts.alice, token_id).is_ok());

        // check balances of contract
        assert_eq!(nft.total_supply(), 1);
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // check enumerable data
        assert_eq!(nft.token_of_owner_by_index(accounts.alice, 0).unwrap(), token_id);
        assert_eq!(nft.token_by_index(0).unwrap(), token_id);
    }

    #[ink::test]
    fn mint_more_tokens() {
        let accounts = accounts();
        let mut nft = PSP721Struct::new();
        let token_ids = vec![[0x1; 32], [0x2; 32], [0x3; 32]];

        assert_eq!(nft.total_supply(), 0);
        assert_eq!(nft.balance_of(accounts.alice), 0);

        // mint tokens to alice
        mint_tokens(&mut nft, &token_ids, accounts.alice);

        // check balances of contract
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 3);
        // check enumerable data
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &token_ids, accounts.alice);
    }

    #[ink::test]
    fn burn_last_token() {
        let accounts = accounts();
        let mut nft = PSP721Struct::new();
        let token_ids = vec![[0x1; 32], [0x2; 32], [0x3; 32]];
        let exp_ids = vec![[0x1; 32], [0x2; 32]];

        // mint tokens to alice
        mint_tokens(&mut nft, &token_ids, accounts.alice);
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 3);
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &token_ids, accounts.alice);

        assert!(nft._burn_from(accounts.alice, token_ids[2]).is_ok());

        // check balances of contract
        assert_eq!(nft.total_supply(), 2);
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // check enumerable data
        compare_tokens(&nft, &exp_ids);
        compare_owner_tokens(&nft, &exp_ids, accounts.alice);
    }

    #[ink::test]
    fn burn_middle_token() {
        let accounts = accounts();
        let mut nft = PSP721Struct::new();
        let token_ids = vec![[0x1; 32], [0x2; 32], [0x3; 32]];
        let exp_ids = vec![[0x1; 32], [0x3; 32]];

        // mint tokens to alice
        mint_tokens(&mut nft, &token_ids, accounts.alice);
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 3);
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &token_ids, accounts.alice);

        assert!(nft._burn_from(accounts.alice, token_ids[1]).is_ok());

        // check balances of contract
        assert_eq!(nft.total_supply(), 2);
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // check enumerable data
        compare_tokens(&nft, &exp_ids);
        compare_owner_tokens(&nft, &exp_ids, accounts.alice);
    }

    #[ink::test]
    fn transfer_last_token() {
        let accounts = accounts();
        let mut nft = PSP721Struct::new();
        let token_ids = vec![[0x1; 32], [0x2; 32], [0x3; 32]];
        let exp_ids_alice = vec![[0x1; 32], [0x2; 32]];
        let exp_ids_bob = vec![[0x3; 32]];

        // mint tokens to alice
        mint_tokens(&mut nft, &token_ids, accounts.alice);
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 3);
        assert_eq!(nft.balance_of(accounts.bob), 0);
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &token_ids, accounts.alice);
        compare_owner_tokens(&nft, &vec![], accounts.bob);

        assert!(nft.transfer(accounts.bob, token_ids[2], Vec::<u8>::new()).is_ok());

        // check balances of contract
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 2);
        assert_eq!(nft.balance_of(accounts.bob), 1);
        // check enumerable data
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &exp_ids_alice, accounts.alice);
        compare_owner_tokens(&nft, &exp_ids_bob, accounts.bob);
    }

    #[ink::test]
    fn transfer_middle_token() {
        let accounts = accounts();
        let mut nft = PSP721Struct::new();
        let token_ids = vec![[0x1; 32], [0x2; 32], [0x3; 32]];
        let exp_ids_alice = vec![[0x1; 32], [0x3; 32]];
        let exp_ids_bob = vec![[0x2; 32]];

        // mint tokens to alice
        mint_tokens(&mut nft, &token_ids, accounts.alice);
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 3);
        assert_eq!(nft.balance_of(accounts.bob), 0);
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &token_ids, accounts.alice);
        compare_owner_tokens(&nft, &vec![], accounts.bob);

        assert!(nft.transfer(accounts.bob, token_ids[1], Vec::<u8>::new()).is_ok());

        // check balances of contract
        assert_eq!(nft.total_supply(), 3);
        assert_eq!(nft.balance_of(accounts.alice), 2);
        assert_eq!(nft.balance_of(accounts.bob), 1);
        // check enumerable data
        compare_tokens(&nft, &token_ids);
        compare_owner_tokens(&nft, &exp_ids_alice, accounts.alice);
        compare_owner_tokens(&nft, &exp_ids_bob, accounts.bob);
    }

    fn mint_tokens(nft: &mut PSP721Struct, token_ids: &Vec<Id>, account: AccountId) {
        for i in 0..token_ids.len() {
            assert!(nft._mint_to(account, token_ids[i]).is_ok());
        }
    }

    fn compare_owner_tokens(nft: &PSP721Struct, token_ids: &Vec<Id>, account: AccountId) {
        for i in 0..token_ids.len() {
            assert_eq!(nft.token_of_owner_by_index(account, i as u32).unwrap(), token_ids[i]);
        }
    }

    fn compare_tokens(nft: &PSP721Struct, token_ids: &Vec<Id>) {
        for i in 0..token_ids.len() {
            assert_eq!(nft.token_by_index(i as u32).unwrap(), token_ids[i]);
        }
    }
}
