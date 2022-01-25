#![feature(min_specialization)]
#[cfg(feature = "psp1155")]
#[brush::contract]
mod psp1155_burnable {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp1155::extensions::burnable::*;
    use ink_lang as ink;

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP1155Internal for PSP1155Struct {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if from.is_some() && from.unwrap() == &self.hated_account {
                return Err(PSP1155Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP1155Burnable for PSP1155Struct {}
    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint_to(acc, vec![(id, amount)])
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }
    }

    #[ink::test]
    fn burn_works() {
        let token_id = [1; 32];
        let token_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, token_amount).is_ok());
        assert!(nft.mint(accounts.bob, token_id, token_amount).is_ok());
        change_caller(accounts.bob);
        assert!(nft.set_approval_for_all(accounts.alice, true).is_ok());
        change_caller(accounts.alice);
        assert_eq!(nft.balance_of(accounts.alice, token_id), token_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id), token_amount);

        assert!(nft.burn(accounts.alice, vec![(token_id, token_amount)]).is_ok());
        assert!(nft.burn(accounts.bob, vec![(token_id, token_amount)]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), 0);
    }

    #[ink::test]
    fn burn_from_without_allowance() {
        let token_id_1 = [1; 32];
        let token_1_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.bob, token_id_1, token_1_amount).is_ok());

        assert_eq!(
            Err(PSP1155Error::NotAllowed),
            nft.burn(accounts.bob, vec![(token_id_1, token_1_amount)])
        );
    }

    #[ink::test]
    fn burn_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 2;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();

        assert_eq!(
            Err(PSP1155Error::InsufficientBalance),
            nft.burn(accounts.alice, vec![(token_id_1, burn_amount)])
        );
    }

    #[ink::test]
    fn burn_from_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 21;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        change_caller(accounts.bob);
        assert!(nft.set_approval_for_all(accounts.alice, true).is_ok());
        change_caller(accounts.alice);

        assert_eq!(
            Err(PSP1155Error::InsufficientBalance),
            nft.burn(accounts.bob, vec![(token_id_1, burn_amount)])
        );
    }

    #[ink::test]
    fn should_not_burn_from_hated_account() {
        let accounts = accounts();
        let token_id_1 = [1; 32];
        let token_1_amount = 1;
        let token_id_2 = [2; 32];
        let token_2_amount = 1;
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id_1, token_1_amount).is_ok());
        assert!(nft.mint(accounts.alice, token_id_2, token_2_amount).is_ok());
        // Alice can burn token from not hated account
        assert!(nft.burn(accounts.alice, vec![(token_id_1, token_1_amount)]).is_ok());
        // Hate Alice account
        nft.set_hated_account(accounts.alice);
        // Alice cannot burn tokens from hated account
        assert_eq!(
            nft.burn(accounts.alice, vec![(token_id_2, token_2_amount)]),
            Err(PSP1155Error::Custom(String::from("I hate this account!")))
        );
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice, token_id_2), token_2_amount);
    }
}
