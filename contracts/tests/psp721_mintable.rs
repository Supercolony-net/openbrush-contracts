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
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP721Internal for PSP721Struct {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP721Error> {
            if to.unwrap() == &self.hated_account {
                return Err(PSP721Error::Custom(String::from("I hate this account!")))
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
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
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
    fn should_not_mint_if_receiver_hated_account() {
        // Constructor works.
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP721Struct::new();
        // Can mint token to Alice while not hated
        assert!(nft.mint(accounts.alice, [1; 32]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Hate Alice account
        nft.set_hated_account(accounts.alice);
        // Cannot mint tokens to hated account
        assert_eq!(
            nft.mint(accounts.alice, [2; 32]),
            Err(PSP721Error::Custom(String::from("I hate this account!")))
        );
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }
}
