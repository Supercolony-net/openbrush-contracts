#[cfg(test)]
#[brush::contract]
mod mintable {
    use brush::test_utils::accounts;
    use ink_lang as ink;
    use psp721::{
        extensions::mintable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
    }

    impl PSP721 for PSP721Struct {}

    impl PSP721Mintable for PSP721Struct {}

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
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
        assert!(nft.mint([1; 32]).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }

    #[ink::test]
    fn mint_existing_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Create token Id 1.
        assert!(nft.mint([1; 32]).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Cannot create  token Id if it _exists.
        assert_eq!(nft.mint([1; 32]), Err(PSP721Error::TokenExists));
        assert_eq!(nft.mint_to(accounts.bob, [1; 32]), Err(PSP721Error::TokenExists));
    }
}
