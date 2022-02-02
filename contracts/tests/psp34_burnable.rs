#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34_burnable {
    use brush::test_utils::accounts;
    use contracts::psp34::extensions::burnable::*;
    use ink_lang as ink;

    #[derive(Default, PSP34Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP34Internal for PSP34Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_before {
                return Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")));
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_after {
                return Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")));
            }
            Ok(())
        }
    }

    impl PSP34 for PSP34Struct {}
    impl PSP34Burnable for PSP34Struct {}

    impl PSP34Struct {
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
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Destroy token Id 1.
        assert!(nft.burn(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of(Id::U8(1u8)), None);
    }

    #[ink::test]
    fn burn_not_existing_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Try burning a non existent token
        assert_eq!(nft.burn(accounts.alice, Id::U8(4u8)), Err(PSP34Error::TokenNotExists));
    }

    #[ink::test]
    fn before_token_transfer_should_fail_burn() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can burn token
        assert!(nft.burn(accounts.alice, Id::U8(1u8)).is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.burn(accounts.alice, Id::U8(2u8)),
            Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_burn() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can burn token
        assert!(nft.burn(accounts.alice, Id::U8(1u8)).is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.burn(accounts.alice, Id::U8(2u8)),
            Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
