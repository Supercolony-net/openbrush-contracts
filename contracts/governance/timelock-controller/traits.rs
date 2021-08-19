pub use access_control::traits::*;
use brush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        Balance,
        Flush,
        Hash,
        InkStorage,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use core::convert::TryFrom;
use ink_env::{
    call::{
        build_call,
        ExecutionInput,
    },
    hash::Blake2x256,
    DefaultEnvironment,
};
use ink_prelude::{
    vec,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
    Lazy,
};
use scale::Encode;
pub use timelock_controller_derive::TimelockControllerStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

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

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct TimelockControllerData {
    pub min_delay: Lazy<Timestamp>,
    pub timestamps: StorageHashMap<OperationId, Timestamp>,
}

declare_storage_trait!(TimelockControllerStorage, TimelockControllerData);

/// The TimelockController error type. Contract will throw one of this errors.
#[derive(strum_macros::AsRefStr)]
pub enum TimelockControllerError {
    InsufficientDelay,
    OperationAlreadyScheduled,
    OperationCannonBeCanceled,
    OperationIsNotReady,
    MissingDependency,
    UnderlyingTransactionReverted,
    CallerMustBeTimeLock,
}

/// Modifier to make a function callable only by a certain role. In
/// addition to checking the sender's role, zero account's role is also
/// considered. Granting a role to zero account is equivalent to enabling
/// this role for everyone.
#[modifier_definition]
pub fn only_role_or_open_role<T, F, ReturnType>(instance: &mut T, body: F, role: RoleType) -> ReturnType
where
    T: AccessControl,
    F: FnOnce(&mut T) -> ReturnType,
{
    if !instance.has_role(role, ZERO_ADDRESS.into()) {
        instance._check_role(&role, &T::env().caller());
    }
    body(instance)
}

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
///
/// This module is used through embedding of `TimelockControllerData`, `AccessControlData` and
/// implementation of `TimelockController`, `TimelockControllerStorage`, `AccessControl` and
/// `AccessControlStorage` traits.
#[brush::trait_definition]
pub trait TimelockController: AccessControl + TimelockControllerStorage + Flush {
    const TIMELOCK_ADMIN_ROLE: RoleType = brush::blake2b_256_as_u32!("TIMELOCK_ADMIN_ROLE");
    const PROPOSER_ROLE: RoleType = brush::blake2b_256_as_u32!("PROPOSER_ROLE");
    const EXECUTOR_ROLE: RoleType = brush::blake2b_256_as_u32!("EXECUTOR_ROLE");

    const DONE_TIMESTAMP: Timestamp = 1;

    /// Returns whether an id correspond to a registered operation. This
    /// includes both Pending, Ready and Done operations.
    #[ink(message)]
    fn is_operation(&self, id: OperationId) -> bool {
        self.get_timestamp(id) > Timestamp::default()
    }

    /// Returns whether an operation is pending or not.
    #[ink(message)]
    fn is_operation_pending(&self, id: OperationId) -> bool {
        self.get_timestamp(id) > Self::DONE_TIMESTAMP
    }

    /// Returns whether an operation is ready or not.
    #[ink(message)]
    fn is_operation_ready(&self, id: OperationId) -> bool {
        let timestamp = self.get_timestamp(id);
        timestamp > Self::DONE_TIMESTAMP && timestamp <= Self::env().block_timestamp()
    }

    /// Returns whether an operation is done or not.
    #[ink(message)]
    fn is_operation_done(&self, id: OperationId) -> bool {
        self.get_timestamp(id) == Self::DONE_TIMESTAMP
    }

    /// Returns the timestamp at with an operation becomes ready (0 for
    /// unset operations, 1 for done operations).
    #[ink(message)]
    fn get_timestamp(&self, id: OperationId) -> Timestamp {
        TimelockControllerStorage::get(self)
            .timestamps
            .get(&id)
            .cloned()
            .unwrap_or(Timestamp::default())
    }

    /// Returns the minimum delay for an operation to become valid.
    ///
    /// This value can be changed by executing an operation that calls `update_delay`.
    #[ink(message)]
    fn get_min_delay(&self) -> Timestamp {
        TimelockControllerStorage::get(self).min_delay.clone()
    }

    /// Returns the identifier of an operation containing a single
    /// transaction.
    #[ink(message)]
    fn hash_operation(&self, transaction: Transaction, predecessor: Option<OperationId>, salt: [u8; 32]) -> Hash {
        self._hash_operation(&transaction, &predecessor, &salt)
    }

    /// Returns the identifier of an operation containing a batch of
    /// transactions.
    #[ink(message)]
    fn hash_operation_batch(
        &self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Hash {
        self._hash_operation_batch(&transactions, &predecessor, &salt)
    }

    /// Schedule an operation containing a single transaction.
    ///
    /// Emits a `CallScheduled` event.
    ///
    /// Node: The caller must have the 'PROPOSER_ROLE' role.
    #[ink(message)]
    #[modifiers(only_role(Self::PROPOSER_ROLE))]
    fn schedule(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) {
        let id = self._hash_operation(&transaction, &predecessor, &salt);

        self._schedule(id, &delay);

        self._emit_call_scheduled_event(id, 0, transaction, predecessor, delay);
    }

    /// Schedule an operation containing a batch of transactions.
    ///
    /// Emits one `CallScheduled` event per transaction in the batch.
    ///
    /// Node: The caller must have the 'PROPOSER_ROLE' role.
    #[ink(message)]
    #[modifiers(only_role(Self::PROPOSER_ROLE))]
    fn schedule_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) {
        let id = self._hash_operation_batch(&transactions, &predecessor, &salt);

        self._schedule(id, &delay);

        for (i, transaction) in transactions.into_iter().enumerate() {
            self._emit_call_scheduled_event(id.clone(), i as u8, transaction, predecessor.clone(), delay.clone());
        }
    }

    /// Cancel an operation.
    ///
    /// Note: the caller must have the 'PROPOSER_ROLE' role.
    #[ink(message)]
    #[modifiers(only_role(Self::PROPOSER_ROLE))]
    fn cancel(&mut self, id: OperationId) {
        assert!(
            self.is_operation_pending(id),
            "{}",
            TimelockControllerError::OperationCannonBeCanceled.as_ref()
        );
        TimelockControllerStorage::get_mut(self).timestamps.take(&id);

        self._emit_cancelled_event(id);
    }

    /// Execute an (ready) operation containing a single transaction.
    ///
    /// Emits a `CallExecuted` event.
    ///
    /// Note: The caller must have the 'EXECUTOR_ROLE' role.
    #[ink(message)]
    #[ink(payable)]
    #[modifiers(only_role_or_open_role(Self::EXECUTOR_ROLE))]
    fn execute(&mut self, transaction: Transaction, predecessor: Option<OperationId>, salt: [u8; 32]) {
        let id = self._hash_operation(&transaction, &predecessor, &salt);

        self._before_call(predecessor);
        self._call(id, 0, transaction);
        self._after_call(id);
    }

    /// Execute an (ready) operation containing a batch of transactions.
    ///
    /// Emits one `CallExecuted` event per transaction in the batch.
    ///
    /// Note: The caller must have the 'EXECUTOR_ROLE' role.
    #[ink(message)]
    #[ink(payable)]
    #[modifiers(only_role_or_open_role(Self::EXECUTOR_ROLE))]
    fn execute_batch(&mut self, transactions: Vec<Transaction>, predecessor: Option<OperationId>, salt: [u8; 32]) {
        let id = self._hash_operation_batch(&transactions, &predecessor, &salt);

        self._before_call(predecessor);

        for (i, transaction) in transactions.into_iter().enumerate() {
            self._call(id, i as u8, transaction);
        }
        self._after_call(id);
    }

    /// Changes the minimum timelock duration for future operations.
    ///
    /// Emits a `MinDelayChange` event.
    ///
    /// Note: The caller must has `TIMELOCK_ADMIN_ROLE` itself.
    /// This can only be achieved by scheduling and later executing
    /// an operation where the timelock is the target and the data is the
    /// ABI-encoded call to this function.
    #[ink(message)]
    fn update_delay(&mut self, new_delay: Timestamp) {
        assert_eq!(
            Self::env().account_id(),
            Self::env().caller(),
            "{}",
            TimelockControllerError::CallerMustBeTimeLock.as_ref()
        );

        let old_delay = Lazy::get(&TimelockControllerStorage::get(self).min_delay).clone();
        self._emit_min_delay_change_event(old_delay, new_delay);

        Lazy::set(&mut TimelockControllerStorage::get_mut(self).min_delay, new_delay);
    }

    // Helper functions

    /// User must override this method in their contract.
    fn _emit_min_delay_change_event(&self, _old_delay: Timestamp, _new_delay: Timestamp) {}

    /// User must override this method in their contract.
    fn _emit_call_scheduled_event(
        &self,
        _id: OperationId,
        _index: u8,
        _transaction: Transaction,
        _predecessor: Option<OperationId>,
        _delay: Timestamp,
    ) {
    }

    /// User must override this method in their contract.
    fn _emit_cancelled_event(&self, _id: OperationId) {}

    /// User must override this method in their contract.
    fn _emit_call_executed_event(&self, _id: OperationId, _index: u8, _transaction: Transaction) {}

    fn _init_with_caller(&mut self, min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) {
        let caller = Self::env().caller();
        TimelockController::_init_with_admin(self, caller, min_delay, proposers, executors);
    }

    fn _init_with_admin(
        &mut self,
        admin: AccountId,
        min_delay: Timestamp,
        proposers: Vec<AccountId>,
        executors: Vec<AccountId>,
    ) {
        self._set_role_admin(Self::TIMELOCK_ADMIN_ROLE, Self::TIMELOCK_ADMIN_ROLE);
        self._set_role_admin(Self::PROPOSER_ROLE, Self::PROPOSER_ROLE);
        self._set_role_admin(Self::EXECUTOR_ROLE, Self::EXECUTOR_ROLE);

        // admin + self administration
        self._setup_role(Self::TIMELOCK_ADMIN_ROLE, Self::env().account_id());
        self._setup_role(Self::TIMELOCK_ADMIN_ROLE, admin);

        // register proposers
        proposers
            .into_iter()
            .for_each(|proposer| self._setup_role(Self::PROPOSER_ROLE, proposer));
        // register executors
        executors
            .into_iter()
            .for_each(|executor| self._setup_role(Self::EXECUTOR_ROLE, executor));

        let old_delay = Lazy::get(&TimelockControllerStorage::get(self).min_delay).clone();
        Lazy::set(&mut TimelockControllerStorage::get_mut(self).min_delay, min_delay);
        self._emit_min_delay_change_event(old_delay, min_delay);
    }

    fn _hash_operation(
        &self,
        transaction: &Transaction,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId {
        let mut hash_data: Vec<u8> = vec![];

        hash_data.append(&mut transaction.encode());
        if predecessor.is_some() {
            hash_data.append(&mut predecessor.unwrap().encode());
        }
        hash_data.append(&mut salt.encode());

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    fn _hash_operation_batch(
        &self,
        transactions: &Vec<Transaction>,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId {
        let mut hash_data: Vec<u8> = vec![];

        hash_data.append(&mut transactions.encode());
        if predecessor.is_some() {
            hash_data.append(&mut predecessor.unwrap().encode());
        }
        hash_data.append(&mut salt.encode());

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    /// Schedule an operation that is to becomes valid after a given delay.
    fn _schedule(&mut self, id: OperationId, delay: &Timestamp) {
        assert!(
            !self.is_operation(id),
            "{}",
            TimelockControllerError::OperationAlreadyScheduled.as_ref()
        );
        assert!(
            delay >= &self.get_min_delay(),
            "{}",
            TimelockControllerError::InsufficientDelay.as_ref()
        );

        TimelockControllerStorage::get_mut(self)
            .timestamps
            .insert(id, Self::env().block_timestamp() + delay);
    }

    /// Checks before execution of an operation's calls.
    fn _before_call(&self, predecessor: Option<OperationId>) {
        assert!(
            predecessor.is_none() || self.is_operation_done(predecessor.unwrap()),
            "{}",
            TimelockControllerError::MissingDependency.as_ref()
        );
    }

    /// Checks after execution of an operation's calls.
    fn _after_call(&mut self, id: OperationId) {
        assert!(
            self.is_operation_ready(id),
            "{}",
            TimelockControllerError::OperationIsNotReady.as_ref()
        );

        TimelockControllerStorage::get_mut(self)
            .timestamps
            .insert(id, Self::DONE_TIMESTAMP);
    }

    /// Execute an operation's call.
    ///
    /// Emits a `CallExecuted` event.
    fn _call(&mut self, id: OperationId, i: u8, transaction: Transaction) {
        // Flush the state into storage before the cross call.
        // Because during cross call we cann call this contract(for example for `update_delay` method).
        self.flush();
        let result: Result<(), TimelockControllerError> = build_call::<DefaultEnvironment>()
            .callee(transaction.callee)
            .gas_limit(transaction.gas_limit)
            .transferred_value(transaction.transferred_value)
            .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(CallInput(&transaction.input)))
            .returns::<()>()
            .fire()
            .map_err(|_| TimelockControllerError::UnderlyingTransactionReverted);

        assert!(result.is_ok(), "{}", result.err().unwrap().as_ref());
        // Load the sate of the contract after the cross call.
        self.load();

        self._emit_call_executed_event(id, i, transaction);
    }
}

/// A wrapper that allows us to encode a blob of bytes.
///
/// We use this to pass the set of untyped (bytes) parameters to the `CallBuilder`.
pub struct CallInput<'a>(&'a [u8]);

impl<'a> scale::Encode for CallInput<'a> {
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(self.0);
    }
}
