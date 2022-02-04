#![feature(min_specialization)]
#[cfg(feature = "psp1155")]
#[brush::contract]
mod psp1155_burnable {
    use brush::test_utils::accounts;
    use contracts::psp1155::extensions::burnable::*;
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
                return Err(PSP1155Error::Custom(String::from("Error on _before_token_transfer")))
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
                return Err(PSP1155Error::Custom(String::from("Error on _after_token_transfer")))
            }
            Ok(())
        }
    }

    impl PSP1155Burnable for PSP1155Struct {}
    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, acc: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint_to(acc, vec![(id, amount)])
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
        let token_id = [1; 32];
        let token_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, token_amount).is_ok());
        assert!(nft.mint(accounts.bob, token_id, token_amount).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id), token_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id), token_amount);

        assert!(nft.burn(accounts.alice, vec![(token_id, token_amount)]).is_ok());
        assert!(nft.burn(accounts.bob, vec![(token_id, token_amount)]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), 0);
    }

    #[ink::test]
    fn burn_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 2;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();

        assert_eq!(
            nft.burn(accounts.alice, vec![(token_id_1, burn_amount)]),
            Err(PSP1155Error::InsufficientBalance),
        );
    }

    #[ink::test]
    fn before_token_transfer_should_fail_burn() {
        let accounts = accounts();
        let token_id = [1; 32];
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, 2).is_ok());
        // Alice can burn tokens
        assert!(nft.burn(accounts.alice, vec![(token_id, 1)]).is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.burn(accounts.alice, vec![(token_id, 1)]),
            Err(PSP1155Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_burn() {
        let accounts = accounts();
        let token_id = [1; 32];
        // Create a new contract instance.
        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, 2).is_ok());
        // Alice can burn tokens
        assert!(nft.burn(accounts.alice, vec![(token_id, 1)]).is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.burn(accounts.alice, vec![(token_id, 1)]),
            Err(PSP1155Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
