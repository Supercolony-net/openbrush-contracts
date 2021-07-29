use crate::stub::PSP721Receiver;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        InkStorage,
        ZERO_ADDRESS,
    },
};
use ink_env::{
    call::FromAccountId,
    Error as Env_error,
};
use ink_lang::ForwardCallMut;
use ink_prelude::{
    collections::{
        btree_map::Entry,
        BTreeMap,
    },
    string::String,
    vec::Vec,
};
use ink_storage::traits::SpreadLayout;
pub use psp721_derive::{
    PSP721MetadataStorage,
    PSP721Storage,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

pub type Id = [u8; 32];

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP721Data {
    pub token_owner: BTreeMap<Id, AccountId>,
    pub token_approvals: BTreeMap<Id, AccountId>,
    pub owned_tokens_count: BTreeMap<AccountId, u32>,
    pub operator_approvals: BTreeMap<(AccountId, AccountId), bool>,
}

declare_storage_trait!(PSP721Storage, PSP721Data);

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP721MetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
}

declare_storage_trait!(PSP721MetadataStorage, PSP721MetadataData);

/// The PSP721 error type. Contract will assert one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum PSP721Error {
    Unknown(String),
    CallFailed,
    NotOwner,
    NotApproved,
    TokenExists,
    TokenNotFound,
    CannotInsert,
    CannotFetchValue,
    NotAllowed,
}

/// Contract module which provides a basic implementation of non fungible token.
///
/// This module is used through embedding of `PSP721Data` and implementation of `IPSP721` and
/// `PSP721Storage` traits.
#[brush::trait_definition]
pub trait IPSP721: PSP721Storage {
    /// Returns the balance of the owner.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32 {
        self.get().owned_tokens_count.get(&owner).cloned().unwrap_or(0)
    }

    /// Returns the owner of the token.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    /// Returns the approved account ID for this token if any.
    #[ink(message)]
    fn get_approved(&self, id: Id) -> Option<AccountId> {
        self.get().token_approvals.get(&id).cloned()
    }

    /// Returns `true` if the operator is approved by the owner.
    #[ink(message)]
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self._approved_for_all(owner, operator)
    }

    /// Approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `ApprovalForAll` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `NotAllowed` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, to: AccountId, approved: bool) {
        self._approve_for_all(to, approved);
    }

    /// Approves the account to transfer the specified token on behalf of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `NotAllowed` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(&mut self, to: AccountId, id: Id) {
        self._approve_for(to, id);
    }

    /// Transfer approved or owned token.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `TokenNotFound` error if `id` is not exist.
    ///
    /// Panics with `NotApproved` error if `from` doesn't have allowance for transferring.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, id: Id) {
        self._transfer_token_from(&from, to.clone(), id);
        self._emit_transfer_event(from, to, id);
    }

    /// Transfers token with `id` from `from` to `to`. Also some `data` can be passed.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `TokenNotFound` error if `id` is not exist.
    ///
    /// Panics with `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Panics with `CallFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, data: Vec<u8>) {
        self._transfer_token_from(&from, to.clone(), id);
        self._call_contract_transfer(Self::env().caller(), from, to, id, data);
        self._emit_transfer_event(from, to, id);
    }

    // Helper functions

    /// Emits transfer event. This method must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: AccountId, _to: AccountId, _id: Id) {}

    /// Emits approval event. This method must be implemented in derived implementation
    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id) {}

    /// Emits approval for all event. This method must be implemented in derived implementation
    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    /// Approves or disapproves the operator to transfer all tokens of the caller.
    fn _approve_for_all(&mut self, to: AccountId, approved: bool) {
        let caller = Self::env().caller();
        assert_ne!(to, caller, "{}", PSP721Error::NotAllowed.as_ref());
        self._emit_approval_for_all_event(caller, to, approved);
        if self._approved_for_all(caller, to) {
            let status = self.get_mut().operator_approvals.get_mut(&(caller, to)).unwrap();
            *status = approved;
        } else {
            self.get_mut().operator_approvals.insert((caller, to), approved);
        }
    }

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Id) {
        let caller = Self::env().caller();
        let owner = self._owner_of(&id);
        if !(owner == Some(caller) || self._approved_for_all(owner.expect("PSP721Error with AccountId"), caller)) {
            panic!("{}", PSP721Error::NotAllowed.as_ref());
        };

        assert!(!to.is_zero(), "{}", PSP721Error::NotAllowed.as_ref());
        assert!(
            self.get_mut().token_approvals.insert(id, to).is_none(),
            "{}",
            PSP721Error::CannotInsert.as_ref()
        );
        self._emit_approval_event(caller, to, id);
    }

    /// Returns the owner of the token.
    fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.get().token_owner.get(id).cloned()
    }

    /// Gets an operator on other Account's behalf.
    fn _approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
        self.get()
            .operator_approvals
            .get(&(owner, operator))
            .unwrap_or(&false)
            .clone()
    }

    /// Returns true if the AccountId `from` is the owner of token `id`
    /// or it has been approved on behalf of the token `id` owner.
    fn _approved_or_owner(&self, from: Option<AccountId>, id: &Id) -> bool {
        let owner = self._owner_of(id);
        !from.unwrap_or_default().is_zero()
            && (from == owner
                || from == self.get().token_approvals.get(id).cloned()
                || self._approved_for_all(
                    owner.expect("PSP721Error with AccountId"),
                    from.expect("PSP721Error with AccountId"),
                ))
    }

    /// Transfers token `id` `from` the sender to the `to` AccountId.
    fn _transfer_token_from(&mut self, from: &AccountId, to: AccountId, id: Id) {
        let caller = Self::env().caller();
        assert!(
            self.get().token_owner.get(&id).is_some(),
            "{}",
            PSP721Error::TokenNotFound.as_ref()
        );
        assert!(
            self._approved_or_owner(Some(caller), &id),
            "{}",
            PSP721Error::NotApproved.as_ref()
        );
        if self.get().token_approvals.contains_key(&id) {
            self.get_mut().token_approvals.remove(&id);
        };
        self._remove_from(from.clone(), id);
        self._add_to(to, id);
    }

    fn _add_to(&mut self, to: AccountId, id: Id) {
        let vacant_token_owner = match self.get_mut().token_owner.entry(id) {
            Entry::Vacant(vacant) => vacant,
            Entry::Occupied(_) => panic!("{}", PSP721Error::TokenExists.as_ref()),
        };
        assert!(!to.is_zero(), "{}", PSP721Error::NotAllowed.as_ref());
        vacant_token_owner.insert(to.clone());
        let entry = self.get_mut().owned_tokens_count.entry(to);
        entry.and_modify(|v| *v += 1).or_insert(1);
    }

    fn _remove_from(&mut self, caller: AccountId, id: Id) {
        let occupied = match self.get_mut().token_owner.entry(id) {
            Entry::Vacant(_) => panic!("{}", PSP721Error::TokenNotFound.as_ref()),
            Entry::Occupied(occupied) => occupied,
        };
        assert_eq!(occupied.get(), &caller, "{}", PSP721Error::NotOwner.as_ref());
        occupied.remove_entry();
        let count = self
            .get_mut()
            .owned_tokens_count
            .get_mut(&caller)
            .expect(PSP721Error::CannotFetchValue.as_ref());
        *count -= 1;
    }

    fn _call_contract_transfer(&self, operator: AccountId, from: AccountId, to: AccountId, id: Id, data: Vec<u8>) {
        let mut receiver: PSP721Receiver = FromAccountId::from_account_id(to);
        match receiver.call_mut().on_psp721_received(operator, from, id, data).fire() {
            Ok(result) => {
                match result {
                    Ok(_) => (),
                    _ => panic!("{}", PSP721Error::CallFailed.as_ref()),
                }
            }
            Err(e) => {
                match e {
                    Env_error::NotCallable => (),
                    _ => panic!("{}", PSP721Error::CallFailed.as_ref()),
                }
            }
        };
    }

    fn _mint(&mut self, id: Id) {
        let to = Self::env().caller();
        self._add_to(to, id.clone());
        self._emit_transfer_event(ZERO_ADDRESS.into(), to, id);
    }

    fn _burn(&mut self, id: Id) {
        let caller = Self::env().caller();
        self._remove_from(caller, id.clone());
        self._emit_transfer_event(caller, ZERO_ADDRESS.into(), id);
    }
}

#[brush::trait_definition]
pub trait IPSP721Metadata: PSP721MetadataStorage {
    /// Returns the token name.
    #[ink(message)]
    fn name(&self) -> Option<String> {
        self.get().name.clone()
    }

    /// Returns the token symbol.
    #[ink(message)]
    fn symbol(&self) -> Option<String> {
        self.get().symbol.clone()
    }

    fn _init_with_metadata(&mut self, name: Option<String>, symbol: Option<String>) {
        self.get_mut().name = name;
        self.get_mut().symbol = symbol;
    }
}

#[brush::trait_definition]
pub trait IPSP721Mint: IPSP721 {
    /// Mints a new token.
    #[ink(message)]
    fn mint(&mut self, id: Id) {
        self._mint(id)
    }

    /// Burns an existing token.
    #[ink(message)]
    fn burn(&mut self, id: Id) {
        self._burn(id)
    }
}

/// The PSP721Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP721ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

/// Handles the receipt of a single PSP-721 token type.
#[brush::trait_definition]
pub trait IPSP721Receiver {
    /// This function is called at the end of a safe_transfer_from after the balance has been updated.
    /// If transfer is rejected it wil return an error.
    #[ink(message)]
    fn on_psp721_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP721ReceiverError>;
}
