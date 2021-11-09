#[cfg(test)]
#[brush::contract]
mod burnable {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use ink_lang as ink;
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
            Self::default()
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
        assert!(nft.burn([1; 32]).is_ok());
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of([1; 32]), None);
    }

    #[ink::test]
    fn burn_fails_token_not_found() {
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Try burning a non existent token
        assert_eq!(nft.burn([4; 32]), Err(PSP721Error::TokenNotExists));
    }

    #[ink::test]
    fn burn_fails_not_owner() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        assert!(nft._mint([1; 32]).is_ok());
        // Try burning this token with a different account
        change_caller(accounts.eve);
        assert_eq!(nft.burn([1; 32]), Err(PSP721Error::NotApproved));
    }
}
