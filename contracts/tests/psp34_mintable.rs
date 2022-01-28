#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34_mintable {
    use brush::test_utils::accounts;
    use contracts::psp34::{
        extensions::mintable::*,
        Id,
    };
    use ink_lang as ink;

    #[derive(Default, PSP34Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
    }

    impl PSP34 for PSP34Struct {}

    impl PSP34Mintable for PSP34Struct {}

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[ink::test]
    fn mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Token 1 does not _exists.
        assert_eq!(nft.owner_of(Id::U8(1u8)), None);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Create token Id 1.
        assert!(nft.mint(Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }

    #[ink::test]
    fn mint_existing_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1.
        assert!(nft.mint(Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Cannot create  token Id if it _exists.
        assert_eq!(nft.mint(Id::U8(1u8)), Err(PSP34Error::TokenExists));
        assert_eq!(nft.mint_to(accounts.bob, Id::U8(1u8)), Err(PSP34Error::TokenExists));
    }
}
