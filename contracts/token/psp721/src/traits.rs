use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        InkStorage,
        ZERO_ADDRESS,
    },
};
pub use common::errors::{
    PSP721Error,
    PSP721ReceiverError,
};
use ink_env::Error as EnvError;
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use psp721_derive::PSP721Storage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

pub type Id = [u8; 32];

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PSP721Data {
    pub token_owner: StorageHashMap<Id, AccountId>,
    pub token_approvals: StorageHashMap<Id, AccountId>,
    pub owned_tokens_count: StorageHashMap<AccountId, u32>,
    pub operator_approvals: StorageHashMap<(AccountId, AccountId), bool>,
}

declare_storage_trait!(PSP721Storage, PSP721Data);

#[brush::wrapper]
pub type PSP721Wrapper = dyn PSP721;

/// Contract module which provides a basic implementation of non fungible token.
///
/// This module is used through embedding of `PSP721Data` and implementation of `PSP721` and
/// `PSP721Storage` traits.
#[brush::trait_definition]
pub trait PSP721: PSP721Storage {
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
    /// Returns `SelfApprove` error if it is self approve.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), PSP721Error> {
        let caller = Self::env().caller();
        self._approve_for_all(caller, operator, approved)?;
        Ok(())
    }

    /// Approves the account to transfer the specified token on behalf of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(&mut self, to: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._approve_for(to, id)?;
        Ok(())
    }

    /// Transfer approved or owned token from caller.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` is not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP721Error> {
        self._transfer_token_from(Self::env().caller(), to, id, data)?;
        Ok(())
    }

    /// Transfer approved or owned token from `from`.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` does not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP721Error> {
        self._transfer_token_from(from, to, id, data)?;
        Ok(())
    }

    // Helper functions

    /// Emits transfer event. This method must be implemented in derived implementation
    fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _id: Id) {}

    /// Emits approval event. This method must be implemented in derived implementation
    fn _emit_approval_event(&self, _from: AccountId, _to: AccountId, _id: Id) {}

    /// Emits approval for all event. This method must be implemented in derived implementation
    fn _emit_approval_for_all_event(&self, _owner: AccountId, _operator: AccountId, _approved: bool) {}

    /// Approves or disapproves the operator to transfer all tokens of the caller.
    fn _approve_for_all(&mut self, owner: AccountId, operator: AccountId, approved: bool) -> Result<(), PSP721Error> {
        if owner == operator {
            return Err(PSP721Error::SelfApprove)
        }
        self.get_mut().operator_approvals.insert((owner, operator), approved);
        self._emit_approval_for_all_event(owner, operator, approved);
        Ok(())
    }

    /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
    fn _approve_for(&mut self, to: AccountId, id: Id) -> Result<(), PSP721Error> {
        let owner = self.get().token_owner.get(&id).cloned();

        if owner.is_none() {
            return Err(PSP721Error::TokenNotExists)
        }
        let owner = owner.unwrap();
        let caller = Self::env().caller();

        if owner == to {
            return Err(PSP721Error::SelfApprove)
        }

        if owner != caller && !self._approved_for_all(owner, caller) {
            return Err(PSP721Error::NotApproved)
        };

        self.get_mut().token_approvals.insert(id, to);
        self._emit_approval_event(caller, to, id);
        Ok(())
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

    /// Transfers token `id` `from` the sender to the `to` AccountId.
    fn _transfer_token_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP721Error> {
        self._before_token_transfer(&from, &to, &id)?;
        self._remove_token(from, &id)?;
        self._do_safe_transfer_check(Self::env().caller(), from, to, id, data)?;
        self._add_token(to.clone(), id.clone())?;
        self._emit_transfer_event(Some(from), Some(to), id);
        Ok(())
    }

    fn _before_token_transfer(&mut self, _from: &AccountId, _to: &AccountId, _id: &Id) -> Result<(), PSP721Error> {
        Ok(())
    }

    /// Child contract can override that if they don't want to do a cross call
    fn _do_safe_transfer_check(
        &self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP721Error> {
        match PSP721ReceiverWrapper::before_received_builder(&to, operator, from, id, data).fire() {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => {
                match e {
                    // `NotCallable` means that the receiver is not a contract.

                    // `CalleeTrapped` means that the receiver has no method called `before_received` or it failed inside.
                    // First case is expected. Second - not. But we can't tell them apart so it is a positive case for now.
                    // https://github.com/paritytech/ink/issues/1002
                    EnvError::NotCallable | EnvError::CalleeTrapped => Ok(()),
                    _ => {
                        Err(PSP721Error::SafeTransferCheckFailed(String::from(
                            "Error during call to receiver",
                        )))
                    }
                }
            }
        }
    }

    fn _add_token(&mut self, to: AccountId, id: Id) -> Result<(), PSP721Error> {
        let to_balance = self.get_mut().owned_tokens_count.get_mut(&to).cloned().unwrap_or(0);
        self.get_mut().owned_tokens_count.insert(to.clone(), to_balance + 1);

        self.get_mut().token_owner.insert(id, to);
        Ok(())
    }

    fn _remove_token(&mut self, from: AccountId, id: &Id) -> Result<(), PSP721Error> {
        let owner = self.get().token_owner.get(id).cloned();

        if owner.is_none() {
            return Err(PSP721Error::TokenNotExists)
        }

        let owner = owner.unwrap();
        let caller = Self::env().caller();
        if owner != caller
            && Some(caller) != self.get().token_approvals.get(id).cloned()
            && !self._approved_for_all(owner, caller)
        {
            return Err(PSP721Error::NotApproved)
        }
        self.get_mut().token_approvals.take(id);
        self.get_mut().token_owner.take(id);

        let from_balance = self.get_mut().owned_tokens_count.get_mut(&from).unwrap().clone();
        self.get_mut().owned_tokens_count.insert(from, from_balance - 1);
        Ok(())
    }

    fn _mint(&mut self, id: Id) -> Result<(), PSP721Error> {
        let to = Self::env().caller();
        self._mint_to(to, id)
    }

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP721Error> {
        if self.get_mut().token_owner.get(&id).is_some() {
            return Err(PSP721Error::TokenExists)
        }
        if to.is_zero() {
            return Err(PSP721Error::MintToZeroAddress)
        }

        self._before_token_transfer(&AccountId::from(ZERO_ADDRESS), &to, &id)?;
        self._add_token(to, id.clone())?;
        self._emit_transfer_event(None, Some(to), id);
        Ok(())
    }

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._before_token_transfer(&from, &AccountId::from(ZERO_ADDRESS), &id)?;
        self._remove_token(from, &id)?;
        self._emit_transfer_event(Some(from), None, id);
        Ok(())
    }

    fn _burn(&mut self, id: Id) -> Result<(), PSP721Error> {
        self._burn_from(Self::env().caller(), id)
    }
}

#[brush::wrapper]
pub type PSP721ReceiverWrapper = dyn PSP721Receiver;

/// PSP721Receiver is a trait for any contract that wants to support safe transfers from a PSP721
/// token smart contract to avoid unexpected tokens in the balance of contract.
/// This method is called before a transfer to ensure the recipient of the tokens acknowledges the receipt.
#[brush::trait_definition]
pub trait PSP721Receiver {
    /// Ensures that the smart contract allows reception of PSP721 token(s).
    /// Returns `Ok(())` if the contract allows the reception of the token(s) and Error `TransferRejected(String))` otherwise.
    ///
    /// This method will get called on every transfer to check whether the recipient in `transfer`
    /// or `transfer_from` is a contract, and if it is, does it accept tokens.
    /// This is done to prevent contracts from locking tokens forever.
    ///
    /// Returns `PSP721ReceiverError` if the contract does not accept the tokens.
    #[ink(message)]
    fn before_received(
        &mut self,
        operator: AccountId,
        from: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP721ReceiverError>;
}
