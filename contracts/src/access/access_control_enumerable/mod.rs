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
    traits::access_control_enumerable::*,
};
pub use derive::{
    AccessControlEnumerableStorage,
    AccessControlStorage,
};
use ink_storage::Mapping;
use openbrush::traits::{
    AccountId,
    Flush,
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::AccessControlEnumerableData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct AccessControlEnumerableData {
    pub role_members: EnumerableMapping,
    pub _reserved: Option<()>,
}

pub trait AccessControlEnumerableStorage: AccessControlStorage + ::openbrush::traits::InkStorage {
    fn get(&self) -> &AccessControlEnumerableData;
    fn get_mut(&mut self) -> &mut AccessControlEnumerableData;
}

impl<T: AccessControlEnumerableStorage + Flush> AccessControlRoleManager for T {
    fn _grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        // default_grant_role(self, role, account)?;

        AccessControlEnumerableStorage::get_mut(self)
            .role_members
            .add(&role, &account)?;

        Ok(())
    }

    fn _revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        default_revoke_role(self, role, account)?;
        AccessControlEnumerableStorage::get_mut(self)
            .role_members
            .remove(&role, &account)?;
        Ok(())
    }
}

impl<T: AccessControlEnumerableStorage + Flush> AccessControlEnumerable for T {
    default fn get_role_member(&self, role: RoleType, index: u128) -> Result<AccountId, AccessControlError> {
        AccessControlEnumerableStorage::get(self).role_members.at(&role, &index)
    }

    default fn get_role_member_count(&self, role: RoleType) -> u128 {
        AccessControlEnumerableStorage::get(self).role_members.length(&role) as u128
    }
}

#[derive(Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate)]
#[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
pub struct EnumerableMapping {
    /// Storage of values
    values: Mapping<RoleType, Vec<AccountId>>,
    /// Mapping from index to `value`.
    /// Position of the value in the `values` array, plus 1
    /// because index 0 means a value is not in the set.
    pub index_to_value: Mapping<(RoleType, u128), AccountId>,
    /// Mapping from `value` to index.
    pub value_to_index: Mapping<(RoleType, AccountId), u128>,
}

impl EnumerableMapping {
    pub fn add(&mut self, member: &RoleType, value: &AccountId) -> Result<(), AccessControlError> {
        if !self.value_to_index.contains((member, value)) {
            let mut values = self.values.get(member).unwrap_or(Vec::<AccountId>::new());
            values.push(*value);
            self.values.insert(member, &values);

            self.index_to_value.insert((member, &self.length(member)), value);
            self.value_to_index.insert((member, value), &self.length(member));
            Ok(())
        } else {
            Err(AccessControlError::ValueAlreadyExists)
        }
    }

    pub fn remove(&mut self, member: &RoleType, value: &AccountId) -> Result<(), AccessControlError> {
        let value_index = self
            .value_to_index
            .get((member, value))
            .ok_or(AccessControlError::ValueNotExists)?;

        let to_delete_index = value_index - 1;
        let last_index = self.length(member) - 1;

        if last_index != to_delete_index {
            let values = self.get_values(member);
            let last_value = values
                .get(last_index as usize)
                .ok_or(AccessControlError::ValueNotExists)?;

            let mut values = self.get_values(member);
            values.insert(to_delete_index as usize, *last_value);
            self.values.insert(member, &values);
            self.index_to_value.insert((member, &value_index), last_value);
            self.value_to_index.insert((member, last_value), &value_index);
        }

        let mut values = self.get_values(member);
        values.remove(last_index as usize);
        self.values.insert(member, &values);
        self.index_to_value.remove((member, &last_index));
        self.value_to_index.remove((member, value));

        Ok(())
    }

    pub fn length(&self, member: &RoleType) -> u128 {
        self.get_values(member).len() as u128
    }

    pub fn at(&self, member: &RoleType, index: &u128) -> Result<AccountId, AccessControlError> {
        self.index_to_value
            .get((member, index))
            .ok_or(AccessControlError::ValueNotExists)
    }

    pub fn get_values(&self, member: &RoleType) -> Vec<AccountId> {
        self.values.get(member).unwrap_or(Vec::<AccountId>::new())
    }
}
