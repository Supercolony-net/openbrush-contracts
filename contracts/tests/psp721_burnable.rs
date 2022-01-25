#![feature(min_specialization)]
#[cfg(feature = "psp721")]
#[brush::contract]
mod psp721_burnable {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp721::extensions::burnable::*;
    use ink_lang as ink;

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP721Internal for PSP721Struct {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP721Error> {
            if from.is_some() && from.unwrap() == &self.hated_account {
                return Err(PSP721Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP721 for PSP721Struct {}
    impl PSP721Burnable for PSP721Struct {}

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
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
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Destroy token Id 1.
        assert!(nft.burn(accounts.alice, [1; 32]).is_ok());
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of([1; 32]), None);
    }

    #[ink::test]
    fn burn_fails_token_not_found() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Try burning a non existent token
        assert_eq!(nft.burn(accounts.alice, [4; 32]), Err(PSP721Error::TokenNotExists));
    }

    #[ink::test]
    fn burn_fails_not_owner() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        // Try burning this token with a different account
        change_caller(accounts.eve);
        assert_eq!(nft.burn(accounts.eve,[1; 32]), Err(PSP721Error::NotApproved));
    }

    #[ink::test]
    fn should_not_burn_from_hated_account() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        assert!(nft._mint([2; 32]).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can burn token from not hated account
        assert!(nft.burn(accounts.alice, [1; 32]).is_ok());
        // Hate Alice account
        nft.set_hated_account(accounts.alice);
        // Alice cannot burn tokens from hated account
        assert_eq!(
            nft.burn(accounts.alice, [2; 32]),
            Err(PSP721Error::Custom(String::from("I hate this account!")))
        );
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }
}
