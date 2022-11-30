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

pub use crate::{
    access_control,
    timelock_controller,
    traits::{
        access_control::*,
        timelock_controller::*,
    },
};
pub use access_control::Internal as _;
pub use timelock_controller::Internal as _;

use core::convert::TryFrom;
use ink::{
    env::{
        call::{
            build_call,
            Call,
            ExecutionInput,
        },
        hash::Blake2x256,
        CallFlags,
        DefaultEnvironment,
    },
    prelude::{
        vec,
        vec::Vec,
    },
    storage::traits::Storable,
};
use openbrush::{
    modifier_definition,
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        Hash,
        OccupiedStorage,
        Storage,
        Timestamp,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub min_delay: Timestamp,
    pub timestamps: Mapping<OperationId, Timestamp>,
    pub _reserved: Option<()>,
}

/// Modifier to make a function callable only by a certain role. In
/// addition to checking the sender's role, zero account's role is also
/// considered. Granting a role to zero account is equivalent to enabling
/// this role for everyone.
#[modifier_definition]
pub fn only_role_or_open_role<T, M, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    M: access_control::members::MembersManager + Storable,
    T: Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if !instance.data().members.has_role(role, &(ZERO_ADDRESS.into())) {
        access_control::check_role(instance, role, T::env().caller())?;
    }
    body(instance)
}

pub const TIMELOCK_ADMIN_ROLE: RoleType = ink::selector_id!("TIMELOCK_ADMIN_ROLE");
pub const PROPOSER_ROLE: RoleType = ink::selector_id!("PROPOSER_ROLE");
pub const EXECUTOR_ROLE: RoleType = ink::selector_id!("EXECUTOR_ROLE");

pub const DONE_TIMESTAMP: Timestamp = 1;

impl<T, M> TimelockController for T
where
    M: access_control::members::MembersManager + Storable,
    T: Storage<Data>,
    T: Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
{
    default fn is_operation(&self, id: OperationId) -> bool {
        self.get_timestamp(id) > Timestamp::default()
    }

    default fn is_operation_pending(&self, id: OperationId) -> bool {
        self.get_timestamp(id) > Self::_done_timestamp()
    }

    default fn is_operation_ready(&self, id: OperationId) -> bool {
        let timestamp = self.get_timestamp(id);
        timestamp > Self::_done_timestamp() && timestamp <= Self::env().block_timestamp()
    }

    default fn is_operation_done(&self, id: OperationId) -> bool {
        self.get_timestamp(id) == Self::_done_timestamp()
    }

    default fn get_timestamp(&self, id: OperationId) -> Timestamp {
        self.data::<Data>().timestamps.get(&id).unwrap_or(Timestamp::default())
    }

    default fn get_min_delay(&self) -> Timestamp {
        self.data::<Data>().min_delay.clone()
    }

    default fn hash_operation(
        &self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Hash {
        self._hash_operation(&transaction, &predecessor, &salt)
    }

    default fn hash_operation_batch(
        &self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Hash {
        self._hash_operation_batch(&transactions, &predecessor, &salt)
    }

    #[modifiers(access_control::only_role(Self::_proposal_role()))]
    default fn schedule(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation(&transaction, &predecessor, &salt);

        self._schedule(id, &delay)?;

        self._emit_call_scheduled_event(id, 0, transaction, predecessor, delay);
        Ok(())
    }

    #[modifiers(access_control::only_role(Self::_proposal_role()))]
    default fn schedule_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
        delay: Timestamp,
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation_batch(&transactions, &predecessor, &salt);

        self._schedule(id, &delay)?;

        for (i, transaction) in transactions.into_iter().enumerate() {
            self._emit_call_scheduled_event(id.clone(), i as u8, transaction, predecessor.clone(), delay.clone());
        }
        Ok(())
    }

    #[modifiers(access_control::only_role(Self::_proposal_role()))]
    default fn cancel(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
        if !self.is_operation_pending(id) {
            return Err(TimelockControllerError::OperationCannonBeCanceled)
        }
        self.data::<Data>().timestamps.remove(&id);

        self._emit_cancelled_event(id);
        Ok(())
    }

    #[modifiers(only_role_or_open_role(Self::_executor_role()))]
    default fn execute(
        &mut self,
        transaction: Transaction,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation(&transaction, &predecessor, &salt);

        self._before_call(predecessor)?;
        self._call(id, 0, transaction)?;
        self._after_call(id)
    }

    #[modifiers(only_role_or_open_role(Self::_executor_role()))]
    default fn execute_batch(
        &mut self,
        transactions: Vec<Transaction>,
        predecessor: Option<OperationId>,
        salt: [u8; 32],
    ) -> Result<(), TimelockControllerError> {
        let id = self._hash_operation_batch(&transactions, &predecessor, &salt);

        self._before_call(predecessor)?;

        for (i, transaction) in transactions.into_iter().enumerate() {
            self._call(id, i as u8, transaction)?;
        }
        self._after_call(id)
    }

    default fn update_delay(&mut self, new_delay: Timestamp) -> Result<(), TimelockControllerError> {
        if Self::env().account_id() != Self::env().caller() {
            return Err(TimelockControllerError::CallerMustBeTimeLock)
        }

        let old_delay = self.data::<Data>().min_delay.clone();
        self._emit_min_delay_change_event(old_delay, new_delay);

        self.data::<Data>().min_delay = new_delay;
        Ok(())
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_min_delay_change_event(&self, _old_delay: Timestamp, _new_delay: Timestamp);
    fn _emit_call_scheduled_event(
        &self,
        _id: OperationId,
        _index: u8,
        _transaction: Transaction,
        _predecessor: Option<OperationId>,
        _delay: Timestamp,
    );
    fn _emit_cancelled_event(&self, _id: OperationId);
    fn _emit_call_executed_event(&self, _id: OperationId, _index: u8, _transaction: Transaction);

    fn _init_with_caller(&mut self, min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>);

    fn _init_with_admin(
        &mut self,
        admin: AccountId,
        min_delay: Timestamp,
        proposers: Vec<AccountId>,
        executors: Vec<AccountId>,
    );

    fn _hash_operation(
        &self,
        transaction: &Transaction,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId;

    fn _hash_operation_batch(
        &self,
        transactions: &Vec<Transaction>,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId;

    /// Schedule an operation that is to becomes valid after a given delay.
    fn _schedule(&mut self, id: OperationId, delay: &Timestamp) -> Result<(), TimelockControllerError>;

    /// Checks before execution of an operation's calls.
    fn _before_call(&self, predecessor: Option<OperationId>) -> Result<(), TimelockControllerError>;

    /// Checks after execution of an operation's calls.
    fn _after_call(&mut self, id: OperationId) -> Result<(), TimelockControllerError>;

    /// Execute an operation's call.
    ///
    /// Emits a `CallExecuted` event.
    fn _call(&mut self, id: OperationId, i: u8, transaction: Transaction) -> Result<(), TimelockControllerError>;

    fn _timelock_admin_role() -> RoleType;

    fn _proposal_role() -> RoleType;

    fn _executor_role() -> RoleType;

    fn _done_timestamp() -> Timestamp;
}

impl<T, M> Internal for T
where
    M: access_control::members::MembersManager + Storable,
    T: Storage<Data>,
    T: Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
{
    default fn _emit_min_delay_change_event(&self, _old_delay: Timestamp, _new_delay: Timestamp) {}
    default fn _emit_call_scheduled_event(
        &self,
        _id: OperationId,
        _index: u8,
        _transaction: Transaction,
        _predecessor: Option<OperationId>,
        _delay: Timestamp,
    ) {
    }
    default fn _emit_cancelled_event(&self, _id: OperationId) {}
    default fn _emit_call_executed_event(&self, _id: OperationId, _index: u8, _transaction: Transaction) {}

    default fn _init_with_caller(
        &mut self,
        min_delay: Timestamp,
        proposers: Vec<AccountId>,
        executors: Vec<AccountId>,
    ) {
        let caller = Self::env().caller();
        Internal::_init_with_admin(self, caller, min_delay, proposers, executors);
    }

    default fn _init_with_admin(
        &mut self,
        admin: AccountId,
        min_delay: Timestamp,
        proposers: Vec<AccountId>,
        executors: Vec<AccountId>,
    ) {
        self._set_role_admin(Self::_timelock_admin_role(), Self::_timelock_admin_role());
        self._set_role_admin(Self::_proposal_role(), Self::_proposal_role());
        self._set_role_admin(Self::_executor_role(), Self::_executor_role());

        // admin + self administration
        self._setup_role(Self::_timelock_admin_role(), Self::env().account_id());
        self._setup_role(Self::_timelock_admin_role(), admin);

        // register proposers
        proposers
            .into_iter()
            .for_each(|proposer| self._setup_role(Self::_proposal_role(), proposer));
        // register executors
        executors
            .into_iter()
            .for_each(|executor| self._setup_role(Self::_executor_role(), executor));

        let old_delay = self.data::<Data>().min_delay.clone();
        self.data::<Data>().min_delay = min_delay;
        self._emit_min_delay_change_event(old_delay, min_delay);
    }

    default fn _hash_operation(
        &self,
        transaction: &Transaction,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId {
        let mut hash_data: Vec<u8> = vec![];

        hash_data.append(&mut scale::Encode::encode(&transaction));
        if predecessor.is_some() {
            hash_data.append(&mut scale::Encode::encode(&predecessor.unwrap()));
        }
        hash_data.append(&mut scale::Encode::encode(&salt));

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    default fn _hash_operation_batch(
        &self,
        transactions: &Vec<Transaction>,
        predecessor: &Option<OperationId>,
        salt: &[u8; 32],
    ) -> OperationId {
        let mut hash_data: Vec<u8> = vec![];

        hash_data.append(&mut scale::Encode::encode(&transactions));
        if predecessor.is_some() {
            hash_data.append(&mut scale::Encode::encode(&predecessor.unwrap()));
        }
        hash_data.append(&mut scale::Encode::encode(&salt));

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    default fn _schedule(&mut self, id: OperationId, delay: &Timestamp) -> Result<(), TimelockControllerError> {
        if self.is_operation(id) {
            return Err(TimelockControllerError::OperationAlreadyScheduled)
        }
        if delay < &self.data::<Data>().min_delay {
            return Err(TimelockControllerError::InsufficientDelay)
        }

        self.data::<Data>()
            .timestamps
            .insert(&id, &(Self::env().block_timestamp() + delay));
        Ok(())
    }

    default fn _before_call(&self, predecessor: Option<OperationId>) -> Result<(), TimelockControllerError> {
        if predecessor.is_some() && !self.is_operation_done(predecessor.unwrap()) {
            return Err(TimelockControllerError::MissingDependency)
        }
        Ok(())
    }

    default fn _after_call(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
        if !self.is_operation_ready(id) {
            return Err(TimelockControllerError::OperationIsNotReady)
        }

        self.data::<Data>().timestamps.insert(&id, &Self::_done_timestamp());
        Ok(())
    }

    default fn _call(
        &mut self,
        id: OperationId,
        i: u8,
        transaction: Transaction,
    ) -> Result<(), TimelockControllerError> {
        // Flush the state into storage before the cross call.
        // Because during cross call we cann call this contract(for example for `update_delay` method).
        self.flush();
        let result = build_call::<DefaultEnvironment>()
            .call_type(
                Call::new()
                    .callee(transaction.callee)
                    .gas_limit(transaction.gas_limit)
                    .transferred_value(transaction.transferred_value),
            )
            .exec_input(ExecutionInput::new(transaction.selector.into()).push_arg(CallInput(&transaction.input)))
            .returns::<()>()
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .map_err(|_| TimelockControllerError::UnderlyingTransactionReverted);

        // Load the sate of the contract after the cross call.
        self.load();

        result?;
        self._emit_call_executed_event(id, i, transaction);
        Ok(())
    }

    default fn _timelock_admin_role() -> RoleType {
        TIMELOCK_ADMIN_ROLE
    }

    default fn _proposal_role() -> RoleType {
        PROPOSER_ROLE
    }

    default fn _executor_role() -> RoleType {
        EXECUTOR_ROLE
    }

    default fn _done_timestamp() -> Timestamp {
        DONE_TIMESTAMP
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
