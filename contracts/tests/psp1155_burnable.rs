#![feature(min_specialization)]
#[cfg(feature = "psp1155")]
#[brush::contract]
mod psp1155_burnable {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp1155::extensions::burnable::*;
    use ink_lang as ink;

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
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
    }

    #[ink::test]
    fn burn_works() {
        let token_id = [1; 32];
        let token_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.alice, token_id, token_amount).is_ok());
        assert!(nft.mint(accounts.bob, token_id, token_amount).is_ok());
        change_caller(accounts.bob);
        assert!(nft.set_approval_for_all(accounts.alice, true).is_ok());
        change_caller(accounts.alice);
        assert_eq!(nft.balance_of(accounts.alice, token_id), token_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id), token_amount);

        assert!(nft.burn(vec![(token_id, token_amount)]).is_ok());
        assert!(nft.burn_from(accounts.bob, vec![(token_id, token_amount)]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id), 0);
    }

    #[ink::test]
    fn burn_from_without_allowance() {
        let token_id_1 = [1; 32];
        let token_1_amount = 20;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        assert!(nft.mint(accounts.bob, token_id_1, token_1_amount).is_ok());

        assert_eq!(
            Err(PSP1155Error::NotAllowed),
            nft.burn_from(accounts.bob, vec![(token_id_1, token_1_amount)])
        );
    }

    #[ink::test]
    fn burn_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 2;

        let mut nft = PSP1155Struct::new();

        assert_eq!(
            Err(PSP1155Error::InsufficientBalance),
            nft.burn(vec![(token_id_1, burn_amount)])
        );
    }

    #[ink::test]
    fn burn_from_insufficient_balance() {
        let token_id_1 = [1; 32];
        let burn_amount = 21;
        let accounts = accounts();

        let mut nft = PSP1155Struct::new();
        change_caller(accounts.bob);
        assert!(nft.set_approval_for_all(accounts.alice, true).is_ok());
        change_caller(accounts.alice);

        assert_eq!(
            Err(PSP1155Error::InsufficientBalance),
            nft.burn_from(accounts.bob, vec![(token_id_1, burn_amount)])
        );
    }
}
