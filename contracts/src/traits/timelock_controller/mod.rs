// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::traits::{
    access_control::*,
    errors::TimelockControllerError,
};
use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    Balance,
    Hash,
    Timestamp,
};

pub type OperationId = Hash;

/// A Transaction is what can be executed by `executor`
#[derive(Default, Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Transaction {
    /// The `AccountId` of the contract that is called in this transaction.
    pub callee: AccountId,
    /// The selector bytes that identifies the function of the callee that should be called.
    pub selector: [u8; 4],
    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,
    /// The amount of chain balance that is transferred to the callee.
    pub transferred_value: Balance,
    /// Gas limit for the execution of the call.
    pub gas_limit: u64,
}

/// TimelockController is AccessControl itself, so creating wrapper for both traits
#[openbrush::wrapper]
pub type TimelockControllerRef = dyn TimelockController + AccessControl;

/// Contract module which acts as a time-locked controller. When set as the
/// owner of an `Ownable` smart contract, it enforces a timelock on all
/// `onlyOwner` maintenance operations. This gives time for users of the
/// controlled contract to exit before a potentially dangerous maintenance
/// operation is applied.
///
/// By default, this contract is self-administered, meaning administration tasks
/// have to go through the timelock process. The proposer (resp executor) role
/// is in charge of proposing (resp executing) operations. A common use case is
/// to position this `TimelockController` as the owner of a smart contract, with
/// a multisig or a DAO as the sole proposer.
#[openbrush::trait_definition]
pub trait TimelockController: AccessControl {
    /// Returns whether an id correspond to a registered operation. This
    /// includes both Pending, Ready and Done operations.
    #[ink(message)]
    fn is_operation(&self, id: OperationId) -> bool;

    /// Returns whether an operation is pending or not.
    #[ink(message)]
    fn is_operation_pending(&self, id: OperationId) -> bool;

    /// Returns whether an operation is ready or not.
    #[ink(message)]
    fn is_operation_ready(&self, id: OperationId) -> bool;

    /// Returns whether an operation is done or not.
    #[ink(message)]
    fn is_operation_done(&self, id: OperationId) -> bool;

    /// Returns the timestamp at with an operation becomes ready (0 for
    /// unset operations, 1 for done operations).
    #[ink(message)]
    fn get_timestamp(&self, id: OperationId) -> Timestamp;

    /// Returns the minimum delay for an operation to become valid.
    ///
    /// This value can be changed by executing an operation that calls `update_delay`.
    #[ink(message)]
    fn get_min_delay(&self) -> Timestamp;

    /// Returns the identifier of an operation containing a single
    /// transaction.
    #[ink(message)]
    fn hash_operation(&self, transaction: Transaction, predecessor: Option<OperationId>, salt: [u8; 32]) -> Hash;

    /// Returns the identifier of an operation containing a batch of
    /// transactions.
    #[ink(message)]
    fn hash_operation_batch(
        &self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Hash;

    /// Schedule an operation containing a single transaction.
    ///
    /// Emits a `CallScheduled` event.
    ///
    /// Node: The caller must have the 'PROPOSER_ROLE' role.
    #[ink(message)]
    fn schedule(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) -> Result<(), TimelockControllerError>;

    /// Schedule an operation containing a batch of transactions.
    ///
    /// Emits one `CallScheduled` event per transaction in the batch.
    ///
    /// Node: The caller must have the 'PROPOSER_ROLE' role.
    #[ink(message)]
    fn schedule_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) -> Result<(), TimelockControllerError>;

    /// Cancel an operation.
    ///
    /// On success a `Cancelled` event is emitted.
    ///
    /// Note: the caller must have the 'PROPOSER_ROLE' role.
    #[ink(message)]
    fn cancel(&mut self, id: OperationId) -> Result<(), TimelockControllerError>;

    /// Execute an (ready) operation containing a single transaction.
    ///
    /// Emits a `CallExecuted` event.
    ///
    /// Note: The caller must have the 'EXECUTOR_ROLE' role.
    #[ink(message, payable)]
    fn execute(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Result<(), TimelockControllerError>;

    /// Execute an (ready) operation containing a batch of transactions.
    ///
    /// Emits one `CallExecuted` event per transaction in the batch.
    ///
    /// Note: The caller must have the 'EXECUTOR_ROLE' role.
    #[ink(message, payable)]
    fn execute_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Result<(), TimelockControllerError>;

    /// Changes the minimum timelock duration for future operations.
    ///
    /// Emits a `MinDelayChange` event.
    ///
    /// Note: The caller must has `TIMELOCK_ADMIN_ROLE` itself.
    /// This can only be achieved by scheduling and later executing
    /// an operation where the timelock is the target and the data is the
    /// ABI-encoded call to this function.
    #[ink(message)]
    fn update_delay(&mut self, new_delay: Timestamp) -> Result<(), TimelockControllerError>;
}
