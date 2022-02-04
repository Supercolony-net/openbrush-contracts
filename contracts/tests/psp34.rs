#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34 {
    use brush::test_utils::{
        accounts,
        change_caller,
    };
    use contracts::psp34::*;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;

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
        id: Id,
    }

    /// Event emitted when an operator is enabled or disabled for an owner.
    /// The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[derive(Default, PSP34Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP34Internal for PSP34Struct {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id) {
            self.env().emit_event(Approval {
                from: _from,
                to: _to,
                id: _id,
            });
        }

        fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner: _owner,
                operator: _operator,
                approved: _approved,
            });
        }

        fn _do_safe_transfer_check(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _to: AccountId,
            _id: Id,
            _data: Vec<u8>,
        ) -> Result<(), PSP34Error> {
            Ok(())
        }

        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_before {
                return Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_after {
                return Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
            }
            Ok(())
        }
    }

    impl PSP34 for PSP34Struct {}

    impl PSP34Struct {
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
    #[should_panic]
    fn collection_id_fails() {
        PSP34Struct::new().collection_id();
    }

    #[ink::test]
    fn transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1 for Alice
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns token 1
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns any token
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // The first Transfer event takes place
        assert_eq!(1, ink_env::test::recorded_events().count());
        // Alice transfers token 1 to Bob
        assert!(nft.transfer(accounts.bob, Id::U8(1u8), vec![]).is_ok());
        // The second Transfer event takes place
        assert_eq!(2, ink_env::test::recorded_events().count());
        // Bob owns token 1
        assert_eq!(nft.balance_of(accounts.bob), 1);
        // Alice doesn't own token 1
        assert_eq!(nft.balance_of(accounts.alice), 0);
    }

    #[ink::test]
    fn not_exist_token_transfer_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Transfer token fails if it does not exists.
        assert_eq!(
            nft.transfer(accounts.bob, Id::U8(1u8), vec![]),
            Err(PSP34Error::TokenNotExists)
        );
        // Token Id 2 does not exists.
        assert_eq!(nft.owner_of(Id::U8(1u8)), None);
    }

    #[ink::test]
    fn not_owned_token_transfer_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 2.
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Token Id 2 is owned by Alice.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        change_caller(accounts.bob);
        // Bob cannot transfer not owned tokens.
        assert_eq!(
            nft.transfer(accounts.eve, Id::U8(1u8), vec![]),
            Err(PSP34Error::NotApproved)
        );
    }

    #[ink::test]
    fn approve_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());

        // Token 1 is not approved
        assert_eq!(nft.get_approved(Id::U8(1u8)), None);

        assert!(nft.approve(accounts.bob, Id::U8(1u8)).is_ok());
        assert_eq!(nft.get_approved(Id::U8(1u8)), Some(accounts.bob));
    }

    #[ink::test]
    fn approve_works_fails() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert_eq!(nft.approve(accounts.bob, Id::U8(1u8)), Err(PSP34Error::TokenNotExists));

        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert_eq!(nft.approve(accounts.alice, Id::U8(1u8)), Err(PSP34Error::SelfApprove));

        change_caller(accounts.bob);
        assert_eq!(nft.approve(accounts.eve, Id::U8(1u8)), Err(PSP34Error::NotApproved));
    }

    #[ink::test]
    fn approved_transfer_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Token Id 1 is owned by Alice.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert!(nft.approve(accounts.bob, Id::U8(1u8)).is_ok());
        // Get contract address.
        change_caller(accounts.bob);
        // Bob transfers token Id 1 from Alice to Eve.
        assert!(nft
            .transfer_from(accounts.alice, accounts.eve, Id::U8(1u8), vec![])
            .is_ok());
        // TokenId 3 is owned by Eve.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.eve));
        // Alice has one token left
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 1 token.
        assert_eq!(nft.balance_of(accounts.eve), 1);
    }

    #[ink::test]
    fn total_supply_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert_eq!(nft.total_supply(), 0);
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(3u8)).is_ok());
        // 3 tokens minted in total
        assert_eq!(nft.total_supply(), 3)
    }

    #[ink::test]
    fn approved_for_all_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert!(nft.set_approval_for_all(accounts.bob, true).is_ok());
        // Bob is an approved operator for Alice
        assert_eq!(nft.is_approved_for_all(accounts.alice, accounts.bob), true);

        change_caller(accounts.bob);
        // Bob transfers token Id 1 from Alice to Eve.
        assert!(nft
            .transfer_from(accounts.alice, accounts.eve, Id::U8(1u8), vec![])
            .is_ok());
        // TokenId 1 is owned by Eve.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.eve));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob transfers token Id 2 from Alice to Eve.
        assert!(nft
            .transfer_from(accounts.alice, accounts.eve, Id::U8(2u8), vec![])
            .is_ok());
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.eve), 2);

        change_caller(accounts.alice);
        // Remove operator approval for Bob on behalf of Alice.
        assert!(nft.set_approval_for_all(accounts.bob, false).is_ok());
        // Bob is not an approved operator for Alice.
        assert_eq!(nft.is_approved_for_all(accounts.alice, accounts.bob), false);
    }

    #[ink::test]
    fn not_approved_transfer_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(2u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve does not owns tokens.
        assert_eq!(nft.balance_of(accounts.eve), 0);
        // Get contract address.
        change_caller(accounts.bob);
        // Eve is not an approved operator by Alice.
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.frank, Id::U8(1u8), vec![]),
            Err(PSP34Error::NotApproved)
        );
    }

    #[ink::test]
    fn before_token_transfer_should_fail_transfer() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(4u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can transfer token
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, Id::U8(1u8), vec![])
            .is_ok());
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, Id::U8(4u8), vec![]),
            Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_transfer() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        assert!(nft._mint_to(accounts.alice, Id::U8(1u8)).is_ok());
        assert!(nft._mint_to(accounts.alice, Id::U8(4u8)).is_ok());
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Alice can transfer token
        assert!(nft
            .transfer_from(accounts.alice, accounts.bob, Id::U8(1u8), vec![])
            .is_ok());
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.bob, Id::U8(4u8), vec![]),
            Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
