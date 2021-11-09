#[cfg(test)]
#[brush::contract]
mod mintable {
    use brush::test_utils::accounts;
    use ink_lang as ink;
    use psp1155::{
        extensions::mintable::*,
        traits::*,
    };

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
    }

    impl PSP1155Mintable for PSP1155Struct {}

    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
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

        assert!(nft.mint(vec![(token_id_1, token_1_amount)]).is_ok());
        assert!(nft.mint_to(accounts.bob, vec![(token_id_2, token_2_amount)]).is_ok());

        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), token_2_amount);
    }
}
