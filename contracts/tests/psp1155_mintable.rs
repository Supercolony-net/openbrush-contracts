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
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP1155Internal for PSP1155Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if self.return_err_on_before {
                return Err(PSP1155Error::Custom(String::from("Error on _before_token_transfer")));
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if self.return_err_on_after {
                return Err(PSP1155Error::Custom(String::from("Error on _after_token_transfer")));
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

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
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
    fn before_token_transfer_should_fail_mint() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        let mut nft = PSP1155Struct::new();
        // Can mint
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert!(nft.mint(accounts.alice, vec![(token_id_1, token_1_amount)]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, vec![(token_id_2, token_2_amount)]),
            Err(PSP1155Error::Custom(String::from("Error on _before_token_transfer")))
        );
        assert_eq!(nft.balance_of(accounts.alice, token_id_2), 0);
    }

    #[ink::test]
    fn after_token_transfer_should_fail_mint() {
        let token_id_1 = [1; 32];
        let token_id_2 = [2; 32];
        let token_1_amount = 1;
        let token_2_amount = 20;
        let accounts = accounts();
        let mut nft = PSP1155Struct::new();
        // Can mint
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert!(nft.mint(accounts.alice, vec![(token_id_1, token_1_amount)]).is_ok());
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, vec![(token_id_2, token_2_amount)]),
            Err(PSP1155Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
