#![feature(min_specialization)]
#[cfg(feature = "psp1155")]
#[brush::contract]
mod psp1155_mintable {
    use brush::test_utils::accounts;
    use contracts::psp1155::extensions::mintable::*;
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
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if to.unwrap() == &self.hated_account {
                return Err(PSP1155Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP1155Mintable for PSP1155Struct {}
    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Struct {
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
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), 0);

        assert!(nft.mint(accounts.alice, vec![(token_id_1, token_1_amount)]).is_ok());
        assert!(nft.mint(accounts.bob, vec![(token_id_2, token_2_amount)]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), token_2_amount);
    }

    #[ink::test]
    fn should_not_mint_to_hated_account() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        // Can mint to not hated account
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert!(nft.mint(accounts.alice, vec![(token_id_1, token_1_amount)]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        // Hate Alice account
        nft.set_hated_account(accounts.alice);
        // Cannot mint to hated account
        assert_eq!(nft.balance_of(accounts.alice, token_id_2), 0);
        assert_eq!(
            nft.mint(accounts.alice, vec![(token_id_2, token_2_amount)]),
            Err(PSP1155Error::Custom(String::from("I hate this account!")))
        );
        assert_eq!(nft.balance_of(accounts.alice, token_id_2), 0);
    }
}
