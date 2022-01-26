#![feature(min_specialization)]
#[cfg(feature = "psp721")]
#[brush::contract]
mod psp721_mintable {
    use brush::test_utils::accounts;
    use contracts::psp721::extensions::mintable::*;
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
    impl PSP721Mintable for PSP721Struct {}

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn change_state_err(&mut self) {
            if self.return_err_on_before {
                self.return_err_on_before = false;
                self.return_err_on_after = true;
            } else {
                self.return_err_on_before = true;
            }
        }
    }

    #[ink::test]
    fn mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Token 1 does not _exists.
        assert_eq!(nft.owner_of([1; 32]), None);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Create token Id 1.
        assert!(nft.mint(accounts.alice, [1; 32]).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }

    #[ink::test]
    fn mint_existing_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Create token Id 1.
        assert!(nft.mint(accounts.alice, [1; 32]).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Cannot create token Id if it _exists.
        assert_eq!(nft.mint(accounts.alice, [1; 32]), Err(PSP721Error::TokenExists));
        assert_eq!(nft.mint(accounts.bob, [1; 32]), Err(PSP721Error::TokenExists));
    }

    #[ink::test]
    fn before_and_after_token_transfer_should_fail_mint() {
        // Constructor works.
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Can mint token to Alice
        assert!(nft.mint(accounts.alice, [1; 32]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Turn on error on _before_token_transfer
        nft.change_state_err();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, [2; 32]),
            Err(PSP721Error::Custom(String::from("Error on _before_token_transfer")))
        );
        // Turn on error on _after_token_transfer
        nft.change_state_err();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, [2; 32]),
            Err(PSP721Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
