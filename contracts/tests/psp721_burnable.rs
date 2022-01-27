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
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP721Internal for PSP721Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP721Error> {
            if self.return_err_on_before {
                return Err(PSP721Error::Custom(String::from("Error on _before_token_transfer")));
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP721Error> {
            if self.return_err_on_after {
                return Err(PSP721Error::Custom(String::from("Error on _after_token_transfer")));
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

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
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
    fn before_token_transfer_should_fail_burn() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        assert!(nft._mint([2; 32]).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can burn token
        assert!(nft.burn(accounts.alice, [1; 32]).is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.burn(accounts.alice, [2; 32]),
            Err(PSP721Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_burn() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        assert!(nft._mint([2; 32]).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can burn token
        assert!(nft.burn(accounts.alice, [1; 32]).is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.burn(accounts.alice, [2; 32]),
            Err(PSP721Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
