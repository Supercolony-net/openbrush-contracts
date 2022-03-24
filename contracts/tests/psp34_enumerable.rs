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
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_env::DefaultEnvironment;
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
    fn mint_enumerable_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns token 1
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns any token
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
        // check token by index
        assert_eq!(nft.token_by_index(0u128), Ok(Id::U8(1u8)));
        
        

        //nft.transfer(accounts.bob, Id::U8(1u8), vec![]);
        // Alice transfers token 1 to Bob
        // assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // // Bob owns token 1
        // assert_eq!(nft.balance_of(accounts.bob), 1);
        // // Alice doesn't own token 1
        // assert_eq!(nft.balance_of(accounts.alice), 0);
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
    fn enumerable_after_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // check Alice token by index
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(1u8)));
        //act. transfer token from alice to bob
        assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        //bob owns token
        assert_eq!(nft.owners_token_by_index(accounts.bob, 0u128), Ok(Id::U8(1u8)));
        //alice does not own token
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Err(PSP34Error::TokenNotExists));
    }
    #[ink::test]
    fn enumerable_after_approved_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Token Id 1 is owned by Alice.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert!(nft.approve(accounts.bob, Some(Id::U8(1u8)), true).is_ok());
        // Get contract address.
        change_caller(accounts.bob);
        // Bob transfers token Id 1 from Alice to Eve.
        assert!(nft.transfer(accounts.eve, Id::U8(1u8), vec![]).is_ok());
        //eve owns token id 1
        assert_eq!(nft.owners_token_by_index(accounts.eve, 0u128), Ok(Id::U8(1u8)));
        //alice owns token id 2
        assert_eq!(nft.owners_token_by_index(accounts.alice, 0u128), Ok(Id::U8(2u8)));
        //bob does not owns any tokens
        assert_eq!(nft.owners_token_by_index(accounts.bob, 0u128), Err(PSP34Error::TokenNotExists));
        //index 1 points to token with id 2
        assert_eq!(nft.token_by_index(1u128), Ok(Id::U8(2u8)));
    }
}
