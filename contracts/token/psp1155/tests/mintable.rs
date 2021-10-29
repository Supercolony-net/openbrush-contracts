#[cfg(test)]
#[brush::contract]
mod mintable {
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

    impl PSP1155 for PSP1155Struct {
        // Don't do cross call in test
        fn _do_safe_transfer_acceptance_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _amount: Balance,
            _data: Vec<u8>,
        ) -> Result<(), psp1155::traits::PSP1155Error> {
            Ok(())
        }

        // Don't do cross call in test
        fn _do_batch_safe_transfer_acceptance_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _ids_to_amounts: Vec<(Id, Balance)>,
            _data: Vec<u8>,
        ) -> Result<(), psp1155::traits::PSP1155Error> {
            Ok(())
        }
    }

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
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        let mut nft = PSP1155Struct::new();
        assert_eq!(nft.balance_of(accounts.alice, token_id_1), 0);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), 0);

        nft.mint(token_id_1, token_1_amount);
        nft.mint_to(accounts.bob, token_id_2, token_2_amount);

        assert_eq!(nft.balance_of(accounts.alice, token_id_1), token_1_amount);
        assert_eq!(nft.balance_of(accounts.bob, token_id_2), token_2_amount);
    }
}
