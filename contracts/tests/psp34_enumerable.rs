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

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    impl PSP34Internal for PSP34Struct {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
            self.env().emit_event(Approval { from, to, id, approved });
        }

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
    fn mint_works() {
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
        // check that bob does not have token by index
        assert_eq!(
            nft.owners_token_by_index(accounts.bob, 0u128),
            Err(PSP34Error::TokenNotExists)
        );
        // token by index 1 does not exists
        assert_eq!(nft.token_by_index(1u128), Err(PSP34Error::TokenNotExists));

        nft.transfer(accounts.bob, Id::U8(1u8), vec![]);
        // Alice transfers token 1 to Bob
        // assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // // Bob owns token 1
        // assert_eq!(nft.balance_of(accounts.bob), 1);
        // // Alice doesn't own token 1
        // assert_eq!(nft.balance_of(accounts.alice), 0);
    }
}
