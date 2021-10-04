#[cfg(test)]
#[brush::contract]
mod tests {
    use psp721::traits::*;
    use brush::traits::ZERO_ADDRESS;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_env::{
        call,
        test,
    };
    use ink_lang as ink;
    use ink_prelude::string::String;

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

    #[derive(Default, PSP721Storage, PSP721MetadataStorage)]
    #[ink(storage)]
    pub struct PSP721Struct {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[PSP721MetadataStorageField]
        metadata: PSP721MetadataData,
    }

    impl IPSP721 for PSP721Struct {
        fn _emit_transfer_event(&self, _from: AccountId, _to: AccountId, _id: Id) {
            self.env().emit_event(Transfer {
                from: Some(_from),
                to: Some(_to),
                id: _id,
            });
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
    }

    impl IPSP721Mint for PSP721Struct {}

    impl IPSP721Metadata for PSP721Struct {}

    impl PSP721Struct {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            instance._init_with_metadata(name, symbol);
            instance
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let nft = PSP721Struct::new(Some(String::from("TOKEN")), Some(String::from("TKN")));

        assert_eq!(nft.name(), Some(String::from("TOKEN")));
        assert_eq!(nft.symbol(), Some(String::from("TKN")));
    }

    #[ink::test]
    fn mint_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Token 1 does not _exists.
        assert_eq!(nft.owner_of([1; 32]), None);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Create token Id 1.
        nft.mint([1; 32]);
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }

    #[ink::test]
    #[should_panic(expected = "TokenExists")]
    fn mint_existing_should_fail() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Create token Id 1.
        nft.mint([1; 32]);
        // The first Transfer event takes place
        assert_eq!(1, ink_env::test::recorded_events().count());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Cannot create  token Id if it _exists.
        // Bob cannot own token Id 1.
        nft.mint([1; 32]);
    }

    #[ink::test]
    fn approved_transfer_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Create token Id 1.
        nft.mint([1; 32]);
        // Token Id 1 is owned by Alice.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        nft.approve(accounts.bob, [1; 32]);
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
        // Create call
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );
        // Bob transfers token Id 1 from Alice to Eve.
        nft.transfer_from(accounts.alice, accounts.eve, [1; 32]);
        // TokenId 3 is owned by Eve.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.eve));
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 1 token.
        assert_eq!(nft.balance_of(accounts.eve), 1);
    }

    #[ink::test]
    fn approved_for_all_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Create token Id 1.
        nft.mint([1; 32]);
        // Create token Id 2.
        nft.mint([2; 32]);
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        nft.set_approval_for_all(accounts.bob, true);
        // Bob is an approved operator for Alice
        assert_eq!(nft.is_approved_for_all(accounts.alice, accounts.bob), true);
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
        // Create call
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Bob as caller
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.bob,
            callee,
            1000000,
            1000000,
            data,
        );
        // Bob transfers token Id 1 from Alice to Eve.
        nft.transfer_from(accounts.alice, accounts.eve, [1; 32]);
        // TokenId 1 is owned by Eve.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.eve));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob transfers token Id 2 from Alice to Eve.
        nft.transfer_from(accounts.alice, accounts.eve, [2; 32]);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.eve), 2);
        // Get back to the parent execution context.
        ink_env::test::pop_execution_context();
        // Remove operator approval for Bob on behalf of Alice.
        nft.set_approval_for_all(accounts.bob, false);
        // Bob is not an approved operator for Alice.
        assert_eq!(nft.is_approved_for_all(accounts.alice, accounts.bob), false);
    }

    #[ink::test]
    fn not_approved_transfer_should_fail() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Create token Id 1.
        nft.mint([1; 32]);
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve does not owns tokens.
        assert_eq!(nft.balance_of(accounts.eve), 0);
        // Get contract address.
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
        // Create call
        let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
        data.push_arg(&accounts.bob);
        // Push the new execution context to set Eve as caller
        ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
            accounts.eve,
            callee,
            1000000,
            1000000,
            data,
        );
        // Eve is not an approved operator by Alice.
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.frank, [1; 32]),
            Err(PSP721Error::NotApproved)
        );
    }

    #[ink::test]
    fn burn_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Create token Id 1 for Alice
        nft.mint([1; 32]);
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Destroy token Id 1.
        nft.burn([1; 32]);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of([1; 32]), None);
    }

    #[ink::test]
    #[should_panic(expected = "TokenNotFound")]
    fn burn_fails_token_not_found() {
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Try burning a non existent token
        nft.burn([1; 32]);
    }

    #[ink::test]
    #[should_panic(expected = "NotOwner")]
    fn burn_fails_not_owner() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = PSP721Struct::new(None, None);
        // Create token Id 1 for Alice
        nft.mint([1; 32]);
        // Try burning this token with a different account
        set_sender(accounts.eve);
        nft.burn([1; 32]);
    }

    fn set_sender(sender: AccountId) {
        let callee = ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
        test::push_execution_context::<Environment>(
            sender,
            callee,
            1000000,
            1000000,
            test::CallData::new(call::Selector::new([0x00; 4])), // dummy
        );
    }
}
