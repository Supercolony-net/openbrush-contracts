#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34_enumerable {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp34::{
        extensions::{
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        Id,
    };
    use ink_lang as ink;
    use ink_storage::traits::SpreadAllocate;

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34EnumerableStorage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34EnumerableStorageField]
        metadata: PSP34EnumerableData,
    }

    impl PSP34Internal for PSP34Struct {
        fn _do_safe_transfer_check(
            &mut self,
            _operator: &AccountId,
            _from: &AccountId,
            _to: &AccountId,
            _id: &Id,
            _data: &Vec<u8>,
        ) -> Result<(), PSP34Error> {
            Ok(())
        }
    }

    impl PSP34 for PSP34Struct {}

    impl PSP34Mintable for PSP34Struct {}

    impl PSP34Burnable for PSP34Struct {}

    impl PSP34Enumerable for PSP34Struct {}

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }

    #[ink::test]
    fn enumerable_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let nft = PSP34Struct::new();
        // check that alice does not have token by index
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(nft.token_by_index(0u128), Err(PSP34Error::TokenNotExists));
    }

    #[ink::test]
    fn enumerable_mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
        // check token by index
        assert_eq!(nft.token_by_index(0u128), Ok(Id::U8(1u8)));
    }

    #[ink::test]
    fn enumerable_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
        // act. transfer token from alice to bob
        assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // bob owns token
        assert_eq!(nft.owners_token_by_index(accounts.bob, 0u128), Ok(Id::U8(1u8)));
        // alice does not own token
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
    }
    #[ink::test]
    fn enumerable_burn_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // alice still owns token id 1
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
        // index 0 points to token with id 1
        assert_eq!(nft.token_by_index(0u128), Ok(Id::U8(1u8)));
        // Destroy token Id 1.
        assert!(nft.burn(accounts.alice, Id::U8(1u8)).is_ok());
        // alice does not owns any tokens
        assert_eq!(
            nft.owners_token_by_index(accounts.alice, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(nft.token_by_index(0u128), Err(PSP34Error::TokenNotExists));
    }
}
