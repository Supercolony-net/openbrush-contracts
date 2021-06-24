use crate::traits::{Id, PSP721Error};
use crate::stub::{PSP721Receiver};
use ink_env::{
    call::{FromAccountId},
    Error as Env_error,
};
use ink_lang::ForwardCallMut;
use ink_storage::collections::{hashmap::Entry};
pub use ink_storage::collections::{HashMap as StorageHashMap};
use brush::{
    traits::{InkStorage, AccountId},
};
use ink_prelude::{string::String, vec::Vec};
pub use psp721_derive::{PSP721Storage, PSP721MetadataStorage};

const ZERO_ADDRESS: [u8; 32] = [0; 32];

#[brush::internal_trait_definition]
pub trait PSP721MetadataStorage: InkStorage {
    fn _name(&self) -> & Option<String>;
    fn _name_mut(&mut self) -> &mut Option<String>;

    fn _symbol(&self) -> & Option<String>;
    fn _symbol_mut(&mut self) -> &mut Option<String>;
}

#[brush::internal_trait_definition]
pub trait PSP721Storage: InkStorage {
    fn _token_owner(&self) -> & StorageHashMap<Id, AccountId>;
    fn _token_owner_mut(&mut self) -> &mut StorageHashMap<Id, AccountId>;

    fn _token_approvals(&self) -> & StorageHashMap<Id, AccountId>;
    fn _token_approvals_mut(&mut self) -> &mut StorageHashMap<Id, AccountId>;

    fn _owned_tokens_count(&self) -> & StorageHashMap<AccountId, u32>;
    fn _owned_tokens_count_mut(&mut self) -> &mut StorageHashMap<AccountId, u32>;

    fn _operator_approvals(&self) -> & StorageHashMap<(AccountId, AccountId), bool>;
    fn _operator_approvals_mut(&mut self) -> &mut StorageHashMap<(AccountId, AccountId), bool>;
}

pub trait PSP721: PSP721Storage {
    /// Emits transfer event. This method must be implemented in derived implementation
    fn emit_transfer_event(&self, _from: AccountId, _to: AccountId, _id: Id) {
        // TODO: Emit events
    }

    /// Emits approval event. This method must be implemented in derived implementation
    fn emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id) {
        // TODO: Emit events
    }

    /// Emits approval for all event. This method must be implemented in derived implementation
    fn emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {
        // TODO: Emit events
    }

    fn balance_of(&self, owner: AccountId) -> u32 {
        self._owned_tokens_count().get(&owner).cloned().unwrap_or(0)
    }

    fn owner_of(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    fn get_approved(&self, id: Id) -> Option<AccountId> {
        self._token_approvals().get(&id).cloned()
    }

    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self._approved_for_all(owner, operator)
    }

    fn set_approval_for_all(&mut self, to: AccountId, approved: bool) {
        self._approve_for_all(to, approved);
    }

    fn approve(&mut self, to: AccountId, id: Id) {
        self._approve_for(to, id);
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
    ) {
        self._transfer_token_from(&from, to.clone(), id);
        self.emit_transfer_event(from, to, id);
    }

    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) {
        self._transfer_token_from(&from, to.clone(), id);
        self._call_contract_transfer(Self::env().caller(), from, to, id, data);
        self.emit_transfer_event(from, to, id);
    }

    // Internal

    /// Approves or disapproves the operator to transfer all tokens of the caller.
    fn _approve_for_all(&mut self, to: AccountId, approved: bool) {
        let caller = Self::env().caller();
        assert_ne!(to, caller, "{}", PSP721Error::NotAllowed.as_ref());
        self.emit_approval_for_all_event(caller, to, approved);
        if self._approved_for_all(caller, to) {
            let status = self
                ._operator_approvals_mut()
                .get_mut(&(caller, to)).unwrap();
            *status = approved;
        } else {
            self._operator_approvals_mut().insert((caller, to), approved);
        }
    }

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Id) {
        let caller = Self::env().caller();
        let owner = self._owner_of(&id);
        if !(owner == Some(caller)
            || self._approved_for_all(owner.expect("PSP721Error with AccountId"), caller))
        {
            panic!("{}", PSP721Error::NotAllowed.as_ref());
        };

        assert_ne!(to, ZERO_ADDRESS.into(), "{}", PSP721Error::NotAllowed.as_ref());
        assert!(self._token_approvals_mut().insert(id, to).is_none(), "{}", PSP721Error::CannotInsert.as_ref());
        self.emit_approval_event(caller, to, id);
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
            owner.expect("PSP721Error with AccountId"),
            from.expect("PSP721Error with AccountId"),
        ))
    }

    /// Transfers token `id` `from` the sender to the `to` AccountId.
    fn _transfer_token_from(&mut self, from: &AccountId, to: AccountId, id: Id) {
        let caller = Self::env().caller();
        assert!(self._token_owner().get(&id).is_some(), "{}", PSP721Error::TokenNotFound.as_ref());
        assert!(self._approved_or_owner(Some(caller), &id), "{}", PSP721Error::NotApproved.as_ref());
        if self._token_approvals().contains_key(&id) {
            self._token_approvals_mut().take(&id);
        };
        self._remove_from(from.clone(), id);
        self._add_to(to, id);
    }

    fn _add_to(&mut self, to: AccountId, id: Id) {
        let vacant_token_owner = match self._token_owner_mut().entry(id) {
            Entry::Vacant(vacant) => vacant,
            Entry::Occupied(_) => panic!("{}", PSP721Error::TokenExists.as_ref()),
        };
        assert_ne!(to, ZERO_ADDRESS.into(), "{}", PSP721Error::NotAllowed.as_ref());
        vacant_token_owner.insert(to.clone());
        let entry = self._owned_tokens_count_mut().entry(to);
        _increase_counter_of(entry);
    }

    fn _remove_from(&mut self, caller: AccountId, id: Id) {
        let occupied = match self._token_owner_mut().entry(id) {
            Entry::Vacant(_) => panic!("{}", PSP721Error::TokenNotFound.as_ref()),
            Entry::Occupied(occupied) => occupied,
        };
        assert_eq!(occupied.get(), &caller, "{}", PSP721Error::NotOwner.as_ref());
        occupied.remove_entry();
        _decrease_counter_of(self._owned_tokens_count_mut(), &caller);
    }

    fn _call_contract_transfer(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) {
        let mut receiver : PSP721Receiver = FromAccountId::from_account_id(to);
        match receiver.call_mut().on_psp721_received(operator, from, id, data)
            .fire()
        {
            Ok(result) => match result {
                Ok(_) => (),
                _ => panic!("{}", PSP721Error::CallFailed.as_ref()),
            },
            Err(e) => match e {
                Env_error::NotCallable => (),
                _ => panic!("{}", PSP721Error::CallFailed.as_ref()),
            },
        };
    }
}

pub trait PSP721Metadata: PSP721MetadataStorage {
    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>) {
        *self._name_mut() = name;
        *self._symbol_mut() = symbol;
    }

    fn name(&self) -> Option<String> {
        self._name().clone()
    }

    fn symbol(&self) -> Option<String> {
        self._symbol().clone()
    }
}

pub trait PSP721Mint: PSP721 {
    fn mint(&mut self, id: Id) {
        let to = Self::env().caller();
        self._add_to(to, id.clone());
        self.emit_transfer_event(ZERO_ADDRESS.into(), to, id);
    }

    fn burn(&mut self, id: Id) {
        let caller = Self::env().caller();
        self._remove_from(caller, id.clone());
        self.emit_transfer_event(caller, ZERO_ADDRESS.into(), id);
    }
}

#[inline]
fn _decrease_counter_of(hmap: &mut StorageHashMap<AccountId, u32>, of: &AccountId) {
    let count = hmap.get_mut(of).expect(PSP721Error::CannotFetchValue.as_ref());
    *count -= 1;
}

#[inline]
fn _increase_counter_of(entry: Entry<AccountId, u32>) {
    entry.and_modify(|v| *v += 1).or_insert(1);
}