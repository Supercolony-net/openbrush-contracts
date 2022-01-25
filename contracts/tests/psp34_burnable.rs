#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34_burnable {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp34::extensions::burnable::*;
    use ink_lang as ink;

    #[derive(Default, PSP34Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
    }

    impl PSP34 for PSP34Struct {}

    impl PSP34Burnable for PSP34Struct {}

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[ink::test]
    fn burn_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Destroy token Id 1.
        assert!(nft.burn([1; 32]).is_ok());
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of([1; 32]), None);
    }

    #[ink::test]
    fn burn_fails_token_not_found() {
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Try burning a non existent token
        assert_eq!(nft.burn([4; 32]), Err(PSP34Error::TokenNotExists));
    }

    #[ink::test]
    fn burn_fails_not_owner() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        // Try burning this token with a different account
        change_caller(accounts.eve);
        assert_eq!(nft.burn([1; 32]), Err(PSP34Error::NotApproved));
    }
}
