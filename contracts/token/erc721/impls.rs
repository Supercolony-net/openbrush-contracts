use core::result::Result;
use crate::traits::{Id, Erc721Error};
use crate::stub::{Erc721Receiver};
use ink_env::{
    call::{FromAccountId},
    Error as Env_error,
};
use ink_lang::ForwardCallMut;
use ink_storage::collections::{hashmap::Entry, HashMap as StorageHashMap};
use utils::{
    traits::{InkStorage, AccountId},
    define_getters,
};
use ink_prelude::{string::String, vec::Vec};

const ZERO_ADDRESS: [u8; 32] = [0; 32];

pub trait Erc721MetadataStorage: InkStorage {
    // Name of the token
    define_getters!(_name, _name_mut, Option<String>);

    // Symbol of the token
    define_getters!(_symbol, _symbol_mut, Option<String>);
}

pub trait Erc721Storage: InkStorage {
    // Mapping from token to owner.
    define_getters!(_token_owner, _token_owner_mut, StorageHashMap<Id, AccountId>);

    // Mapping from token to approvals users.
    define_getters!(_token_approvals, _token_approvals_mut, StorageHashMap<Id, AccountId>);

    // Mapping from owner to number of owned token.
    define_getters!(_owned_tokens_count, _owned_tokens_count_mut, StorageHashMap<AccountId, u32>);

    // Mapping from owner to operator approvals.
    define_getters!(_operator_approvals, _operator_approvals_mut, StorageHashMap<(AccountId, AccountId), bool>);
}

pub trait Erc721: Erc721Internal {
    fn _balance_of(&self, owner: AccountId) -> u32 {
        self._owned_tokens_count().get(&owner).cloned().unwrap_or(0)
    }

    // It is implemented in Erc721Internal
    // fn _owner_of(&self, id: Id) -> Option<AccountId>;

    fn _get_approved(&self, id: Id) -> Option<AccountId> {
        self._token_approvals().get(&id).cloned()
    }

    fn _is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self._approved_for_all(owner, operator)
    }

    fn _set_approval_for_all(&mut self, to: AccountId, approved: bool) -> Result<(), Erc721Error> {
        self._approve_for_all(to, approved)?;
        Ok(())
    }

    fn _approve(&mut self, to: AccountId, id: Id) -> Result<(), Erc721Error> {
        self._approve_for(to, id)?;
        Ok(())
    }

    fn _transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
    ) -> Result<(), Erc721Error> {
        self._transfer_token_from(&from, &to, id)?;
        self._emit_transfer_event(from, to, id);
        Ok(())
    }

    fn _safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), Erc721Error> {
        self._transfer_token_from(&from, &to, id)?;

        assert_eq!(
            self._call_contract_transfer(Self::env().caller(), from, to, id, data)
                .unwrap(),
            (),
            "Failed contract call during transfer"
        );

        self._emit_transfer_event(from, to, id);
        Ok(())
    }
}

pub trait Erc721Metadata: Erc721MetadataStorage {
    fn new(name: Option<String>, symbol: Option<String>) -> Self {
        let mut instance = Self::_empty();
        *instance._name_mut() = name;
        *instance._symbol_mut() = symbol;
        instance
    }

    // It is available in Erc721MetadataStorage
    // fn _name(&self) -> Option<String>;

    // It is available in Erc721MetadataStorage
    // fn _symbol(&self) -> Option<String>;
}

pub trait Erc721Mint: Erc721Internal {
    fn _mint(&mut self, id: Id) -> Result<(), Erc721Error> {
        let caller = Self::env().caller();
        self._add_token_to(caller, id)?;
        self._emit_transfer_event(ZERO_ADDRESS.into(), caller, id);
        Ok(())
    }

    fn _burn(&mut self, id: Id) -> Result<(), Erc721Error> {
        let caller = Self::env().caller();
        let occupied = match self._token_owner_mut().entry(id) {
            Entry::Vacant(_) => return Err(Erc721Error::TokenNotFound),
            Entry::Occupied(occupied) => occupied,
        };
        if occupied.get() != &caller {
            return Err(Erc721Error::NotOwner);
        };
        occupied.remove_entry();
        _decrease_counter_of(self._owned_tokens_count_mut(), &caller)?;

        self._emit_transfer_event(caller, ZERO_ADDRESS.into(), id);
        Ok(())
    }
}

pub trait Erc721Internal: Erc721Storage {
    /// Emits transfer event. This method must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: AccountId, _to: AccountId, _id: Id);

    /// Emits approval event. This method must be implemented in derived implementation
    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id);

    /// Emits approval for all event. This method must be implemented in derived implementation
    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool);

    /// Adds the token `id` to the `to` AccountID.
    fn _add_token_to(&mut self, to: AccountId, id: Id) -> Result<(), Erc721Error> {
        let vacant_token_owner = match self._token_owner_mut().entry(id) {
            Entry::Vacant(vacant) => vacant,
            Entry::Occupied(_) => return Err(Erc721Error::TokenExists),
        };
        if to == ZERO_ADDRESS.into() {
            return Err(Erc721Error::NotAllowed);
        };
        vacant_token_owner.insert(to);
        let entry = self._owned_tokens_count_mut().entry(to.clone());
        _increase_counter_of(entry);
        Ok(())
    }

    /// Approves or disapproves the operator to transfer all tokens of the caller.
    fn _approve_for_all(&mut self, to: AccountId, approved: bool) -> Result<(), Erc721Error> {
        let caller = Self::env().caller();
        if to == caller {
            return Err(Erc721Error::NotAllowed);
        }
        self._emit_approval_for_all_event(caller, to, approved);
        if self._approved_for_all(caller, to) {
            let status = self
                ._operator_approvals_mut()
                .get_mut(&(caller, to))
                .ok_or(Erc721Error::CannotFetchValue)?;
            *status = approved;
            Ok(())
        } else {
            match self._operator_approvals_mut().insert((caller, to), approved) {
                Some(_) => Err(Erc721Error::CannotInsert),
                None => Ok(()),
            }
        }
    }

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Id) -> Result<(), Erc721Error> {
        let caller = Self::env().caller();
        let owner = self._owner_of(&id);
        if !(owner == Some(caller)
            || self._approved_for_all(owner.expect("Erc721Error with AccountId"), caller))
        {
            return Err(Erc721Error::NotAllowed);
        };
        if to == ZERO_ADDRESS.into() {
            return Err(Erc721Error::NotAllowed);
        };

        if self._token_approvals_mut().insert(id, to).is_some() {
            return Err(Erc721Error::CannotInsert);
        };
        self._emit_approval_event(caller, to, id);
        Ok(())
    }

    /// Removes existing approval from token `id`.
    fn _clear_approval(&mut self, id: &Id) -> Result<(), Erc721Error> {
        if !self._token_approvals().contains_key(id) {
            return Ok(());
        };
        match self._token_approvals_mut().take(id) {
            Some(_res) => Ok(()),
            None => Err(Erc721Error::CannotRemove),
        }
    }

    /// Returns the owner of the token.
    fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self._token_owner().get(id).cloned()
    }

    /// Gets an operator on other Account's behalf.
    fn _approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self
            ._operator_approvals()
            .get(&(owner, operator))
            .unwrap_or(&false).clone()
    }

    /// Returns true if the AccountId `from` is the owner of token `id`
    /// or it has been approved on behalf of the token `id` owner.
    fn _approved_or_owner(&self, from: Option<AccountId>, id: &Id) -> bool {
        let owner = self._owner_of(id);
        from != Some(ZERO_ADDRESS.into())
            && (from == owner
            || from == self._token_approvals().get(id).cloned()
            || self._approved_for_all(
            owner.expect("Erc721Error with AccountId"),
            from.expect("Erc721Error with AccountId"),
        ))
    }

    /// Returns true if token `id` exists or false if it does not.
    fn _exists(&self, id: &Id) -> bool {
        self._token_owner().get(id).is_some() && self._token_owner().contains_key(id)
    }

    fn _call_contract_transfer(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), Erc721Error> {
        let mut receiver : Erc721Receiver = FromAccountId::from_account_id(to);
        match receiver.call_mut().on_erc721_received(operator, from, id, data)
            .fire()
        {
            Ok(result) => match result {
                Ok(_) => Ok(()),
                _ => Err(Erc721Error::CallFailed),
            },
            Err(e) => match e {
                Env_error::NotCallable => Ok(()),
                _ => Err(Erc721Error::CallFailed),
            },
        }
    }

    /// Transfers token `id` `from` the sender to the `to` AccountId.
    fn _transfer_token_from(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: Id,
    ) -> Result<(), Erc721Error> {
        let caller = Self::env().caller();
        if !self._exists(&id) {
            return Err(Erc721Error::TokenNotFound);
        };
        if !self._approved_or_owner(Some(caller), &id) {
            return Err(Erc721Error::NotApproved);
        };
        self._clear_approval(&id)?;
        self._remove_token_from(from, id)?;
        self._add_token_to(to.clone(), id)?;
        Ok(())
    }

    /// Removes token `id` from the owner.
    fn _remove_token_from(&mut self, from: &AccountId, id: Id) -> Result<(), Erc721Error> {
        let occupied = match self._token_owner_mut().entry(id) {
            Entry::Vacant(_) => return Err(Erc721Error::TokenNotFound),
            Entry::Occupied(occupied) => occupied,
        };
        occupied.remove_entry();
        _decrease_counter_of(self._owned_tokens_count_mut(), from)?;
        Ok(())
    }
}

fn _decrease_counter_of(hmap: &mut StorageHashMap<AccountId, u32>, of: &AccountId) -> Result<(), Erc721Error> {
    let count = hmap.get_mut(of).ok_or(Erc721Error::CannotFetchValue)?;
    *count -= 1;
    Ok(())
}

fn _increase_counter_of(entry: Entry<AccountId, u32>) {
    entry.and_modify(|v| *v += 1).or_insert(1);
}

#[cfg(test)]
#[ink_lang::contract]
mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink_env::{call, test};
    use ink_lang as ink;
    use utils::{
        traits::{InkStorage},
        iml_getters,
    };
    use ink::{Env, EmitEvent};
    use crate::traits::{ IErc721, IErc721Mint, IErc721Metadata };

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

    #[derive(Default)]
    #[ink(storage)]
    pub struct Erc721Struct {
        /// Name of the token
        name: Option<String>,
        /// Symbol of the token
        symbol: Option<String>,
        /// Mapping from token to owner.
        token_owner: StorageHashMap<Id, AccountId>,
        /// Mapping from token to approvals users.
        token_approvals: StorageHashMap<Id, AccountId>,
        /// Mapping from owner to number of owned token.
        owned_tokens_count: StorageHashMap<AccountId, u32>,
        /// Mapping from owner to operator approvals.
        operator_approvals: StorageHashMap<(AccountId, AccountId), bool>,
    }

    impl InkStorage for Erc721Struct {}
    impl Erc721Storage for Erc721Struct {
        iml_getters!(token_owner, _token_owner, _token_owner_mut, StorageHashMap<Id, AccountId>);
        iml_getters!(token_approvals, _token_approvals, _token_approvals_mut, StorageHashMap<Id, AccountId>);
        iml_getters!(owned_tokens_count, _owned_tokens_count, _owned_tokens_count_mut, StorageHashMap<AccountId, u32>);
        iml_getters!(operator_approvals, _operator_approvals, _operator_approvals_mut, StorageHashMap<(AccountId, AccountId), bool>);
    }

    impl Erc721Internal for Erc721Struct {
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
    impl Erc721 for Erc721Struct {}
    impl Erc721Mint for Erc721Struct {}

    impl Erc721MetadataStorage for Erc721Struct {
        iml_getters!(name, _name, _name_mut, Option<String>);
        iml_getters!(symbol, _symbol, _symbol_mut, Option<String>);
    }
    impl Erc721Metadata for Erc721Struct {}
    
    impl IErc721 for Erc721Struct {
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self._balance_of(owner)
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self._owner_of(&id)
        }

        #[ink(message)]
        fn get_approved(&self, id: Id) -> Option<AccountId> {
            self._get_approved(id)
        }

        #[ink(message)]
        fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self._is_approved_for_all(owner, operator)
        }

        #[ink(message)]
        fn set_approval_for_all(&mut self, to: AccountId, approved: bool) -> Result<(), Erc721Error> {
            self._set_approval_for_all(to, approved)
        }

        #[ink(message)]
        fn approve(&mut self, to: AccountId, id: Id) -> Result<(), Erc721Error> {
            self._approve(to, id)
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
        ) -> Result<(), Erc721Error> {
            self._transfer_from(from, to, id)
        }

        #[ink(message)]
        fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
            data: Vec<u8>,
        ) -> Result<(), Erc721Error> {
            self._safe_transfer_from(from, to, id, data)
        }
    }

    impl IErc721Mint for Erc721Struct {
        #[ink(message)]
        fn mint(&mut self, id: Id) -> Result<(), Erc721Error> {
            self._mint(id)
        }

        #[ink(message)]
        fn burn(&mut self, id: Id) -> Result<(), Erc721Error> {
            self._burn(id)
        }
    }

    impl IErc721Metadata for Erc721Struct {
        #[ink(message)]
        fn name(&self) -> Option<String> {
            self._name().clone()
        }

        #[ink(message)]
        fn symbol(&self) -> Option<String> {
            self._symbol().clone()
        }
    }

    impl Erc721Struct {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            <Erc721Struct as Erc721Metadata>::new(name, symbol)
        }
    }

    #[ink::test]
    fn init_with_name_and_symbol_works() {
        let nft = Erc721Struct::new(Some(String::from("TOKEN")), Some(String::from("TKN")));

        assert_eq!(nft.name(), Some(String::from("TOKEN")));
        assert_eq!(nft.symbol(), Some(String::from("TKN")));
    }

    #[ink::test]
    fn mint_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Token 1 does not _exists.
        assert_eq!(nft.owner_of([1; 32]), None);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Create token Id 1.
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }

    #[ink::test]
    fn mint_existing_should_fail() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Create token Id 1.
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // The first Transfer event takes place
        assert_eq!(1, ink_env::test::recorded_events().count());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Cannot create  token Id if it _exists.
        // Bob cannot own token Id 1.
        assert_eq!(nft.mint([1; 32]), Err(Erc721Error::TokenExists));
    }

    #[ink::test]
    fn approved_transfer_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Create token Id 1.
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // Token Id 1 is owned by Alice.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert_eq!(nft.approve(accounts.bob, [1; 32]), Ok(()));
        // Get contract address.
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
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
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.eve, [1; 32]),
            Ok(())
        );
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
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Create token Id 1.
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // Create token Id 2.
        assert_eq!(nft.mint([2; 32]), Ok(()));
        // Alice owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.alice), 2);
        // Approve token Id 1 transfer for Bob on behalf of Alice.
        assert_eq!(nft.set_approval_for_all(accounts.bob, true), Ok(()));
        // Bob is an approved operator for Alice
        assert_eq!(
            nft.is_approved_for_all(accounts.alice, accounts.bob),
            true
        );
        // Get contract address.
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
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
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.eve, [1; 32]),
            Ok(())
        );
        // TokenId 1 is owned by Eve.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.eve));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob transfers token Id 2 from Alice to Eve.
        assert_eq!(
            nft.transfer_from(accounts.alice, accounts.eve, [2; 32]),
            Ok(())
        );
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve owns 2 tokens.
        assert_eq!(nft.balance_of(accounts.eve), 2);
        // Get back to the parent execution context.
        ink_env::test::pop_execution_context();
        // Remove operator approval for Bob on behalf of Alice.
        assert_eq!(nft.set_approval_for_all(accounts.bob, false), Ok(()));
        // Bob is not an approved operator for Alice.
        assert_eq!(
            nft.is_approved_for_all(accounts.alice, accounts.bob),
            false
        );
    }

    #[ink::test]
    fn not_approved_transfer_should_fail() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Create token Id 1.
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve does not owns tokens.
        assert_eq!(nft.balance_of(accounts.eve), 0);
        // Get contract address.
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
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
            Err(Erc721Error::NotApproved)
        );
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Bob does not owns tokens.
        assert_eq!(nft.balance_of(accounts.bob), 0);
        // Eve does not owns tokens.
        assert_eq!(nft.balance_of(accounts.eve), 0);
    }

    #[ink::test]
    fn burn_works() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Create token Id 1 for Alice
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of([1; 32]), Some(accounts.alice));
        // Destroy token Id 1.
        assert_eq!(nft.burn([1; 32]), Ok(()));
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Token Id 1 does not _exists
        assert_eq!(nft.owner_of([1; 32]), None);
    }

    #[ink::test]
    fn burn_fails_token_not_found() {
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Try burning a non existent token
        assert_eq!(nft.burn([1; 32]), Err(Erc721Error::TokenNotFound));
    }

    #[ink::test]
    fn burn_fails_not_owner() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        // Create a new contract instance.
        let mut nft = Erc721Struct::new(None, None);
        // Create token Id 1 for Alice
        assert_eq!(nft.mint([1; 32]), Ok(()));
        // Try burning this token with a different account
        set_sender(accounts.eve);
        assert_eq!(nft.burn([1; 32]), Err(Erc721Error::NotOwner));
    }

    fn set_sender(sender: AccountId) {
        let callee =
            ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or(ZERO_ADDRESS.into());
        test::push_execution_context::<Environment>(
            sender,
            callee,
            1000000,
            1000000,
            test::CallData::new(call::Selector::new([0x00; 4])), // dummy
        );
    }
}