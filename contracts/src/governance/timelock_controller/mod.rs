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
    access_control::*,
    traits::timelock_controller::*,
};
use core::convert::TryFrom;
pub use derive::TimelockControllerStorage;
use ink_env::{
    call::{
        build_call,
        Call,
        ExecutionInput,
    },
    hash::Blake2x256,
    CallFlags,
    DefaultEnvironment,
};
use ink_prelude::{
    vec,
    vec::Vec,
};
use openbrush::{
    declare_storage_trait,
    modifier_definition,
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        Flush,
        Hash,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use scale::Encode;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::TimelockControllerData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct TimelockControllerData {
    pub access_control: AccessControlData,
    pub min_delay: Timestamp,
    pub timestamps: Mapping<OperationId, Timestamp>,
    pub _reserved: Option<()>,
}

declare_storage_trait!(TimelockControllerStorage);

impl<T: TimelockControllerStorage<Data = TimelockControllerData>> AccessControlStorage for T {
    type Data = AccessControlData;

    fn get(&self) -> &Self::Data {
        &T::get(self).access_control
    }

    fn get_mut(&mut self) -> &mut Self::Data {
        &mut T::get_mut(self).access_control
    }
}

/// Modifier to make a function callable only by a certain role. In
/// addition to checking the sender's role, zero account's role is also
/// considered. Granting a role to zero account is equivalent to enabling
/// this role for everyone.
#[modifier_definition]
pub fn only_role_or_open_role<T, B, F, R, E>(instance: &mut T, body: F, role: RoleType) -> Result<R, E>
where
    B: AccessControlMemberManager,
    T: AccessControlStorage<Data = AccessControlData<B>>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<AccessControlError>,
{
    if !instance.get().members.has_role(role, &ZERO_ADDRESS.into()) {
        check_role(instance, role, T::env().caller())?;
    }
    body(instance)
}

pub const TIMELOCK_ADMIN_ROLE: RoleType = ink_lang::selector_id!("TIMELOCK_ADMIN_ROLE");
pub const PROPOSER_ROLE: RoleType = ink_lang::selector_id!("PROPOSER_ROLE");
pub const EXECUTOR_ROLE: RoleType = ink_lang::selector_id!("EXECUTOR_ROLE");

pub const DONE_TIMESTAMP: Timestamp = 1;

impl<T: TimelockControllerStorage<Data = TimelockControllerData> + Flush> TimelockController for T {
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
        TimelockControllerStorage::get(self)
            .timestamps
            .get(&id)
            .unwrap_or(Timestamp::default())
    }

    default fn get_min_delay(&self) -> Timestamp {
        TimelockControllerStorage::get(self).min_delay.clone()
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

    #[modifiers(only_role(Self::_proposal_role()))]
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

    #[modifiers(only_role(Self::_proposal_role()))]
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

    #[modifiers(only_role(Self::_proposal_role()))]
    default fn cancel(&mut self, id: OperationId) -> Result<(), TimelockControllerError> {
        if !self.is_operation_pending(id) {
            return Err(TimelockControllerError::OperationCannonBeCanceled)
        }
        TimelockControllerStorage::get_mut(self).timestamps.remove(&id);

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

        let old_delay = TimelockControllerStorage::get(self).min_delay.clone();
        self._emit_min_delay_change_event(old_delay, new_delay);

        TimelockControllerStorage::get_mut(self).min_delay = new_delay;
        Ok(())
    }
}

pub trait TimelockControllerInternal {
    /// User must override this method in their contract.
    fn _emit_min_delay_change_event(&self, _old_delay: Timestamp, _new_delay: Timestamp);

    /// User must override this method in their contract.
    fn _emit_call_scheduled_event(
        &self,
        _id: OperationId,
        _index: u8,
        _transaction: Transaction,
        _predecessor: Option<OperationId>,
        _delay: Timestamp,
    );

    /// User must override this method in their contract.
    fn _emit_cancelled_event(&self, _id: OperationId);

    /// User must override this method in their contract.
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

impl<T: TimelockControllerStorage<Data = TimelockControllerData> + Flush> TimelockControllerInternal for T {
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
        TimelockControllerInternal::_init_with_admin(self, caller, min_delay, proposers, executors);
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

        let old_delay = TimelockControllerStorage::get(self).min_delay.clone();
        TimelockControllerStorage::get_mut(self).min_delay = min_delay;
        self._emit_min_delay_change_event(old_delay, min_delay);
    }

    default fn _hash_operation(
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

    default fn _hash_operation_batch(
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

    default fn _schedule(&mut self, id: OperationId, delay: &Timestamp) -> Result<(), TimelockControllerError> {
        if self.is_operation(id) {
            return Err(TimelockControllerError::OperationAlreadyScheduled)
        }
        if delay < &TimelockControllerStorage::get(self).min_delay {
            return Err(TimelockControllerError::InsufficientDelay)
        }

        TimelockControllerStorage::get_mut(self)
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

        TimelockControllerStorage::get_mut(self)
            .timestamps
            .insert(&id, &Self::_done_timestamp());
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
