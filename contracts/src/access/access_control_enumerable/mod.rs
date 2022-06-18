// // Copyright (c) 2012-2022 Supercolony
// //
// // Permission is hereby granted, free of charge, to any person obtaining
// // a copy of this software and associated documentation files (the"Software"),
// // to deal in the Software without restriction, including
// // without limitation the rights to use, copy, modify, merge, publish,
// // distribute, sublicense, and/or sell copies of the Software, and to
// // permit persons to whom the Software is furnished to do so, subject to
// // the following conditions:
// //
// // The above copyright notice and this permission notice shall be
// // included in all copies or substantial portions of the Software.
// //
// // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// // EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// // MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// // NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// // LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// // OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// // WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// pub use crate::{
//     access_control::*,
//     traits::access_control_enumerable::*,
// };
// pub use derive::{
//     AccessControlEnumerableStorage,
//     AccessControlStorage,
// };
// use ink_prelude::vec::Vec;
// use openbrush::{
//     declare_storage_trait,
//     storage::{
//         Mapping,
//         MultiMapping,
//         TypeGuard,
//     },
//     traits::{
//         AccountId,
//         Flush,
//     },
// };

// pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::AccessControlEnumerableData");

// #[derive(Default, Debug)]
// #[openbrush::storage(STORAGE_KEY)]
// pub struct AccessControlEnumerableData {
//     pub values: Mapping<RoleType, Vec<AccountId>, RoleTypeKey>,
//     pub role_members: MultiMapping<RoleType, AccountId, RoleTypeKey /* for optimization */>,
//     pub _reserved: Option<()>,
// }

// pub struct RoleTypeKey;

// impl<'a> TypeGuard<'a> for RoleTypeKey {
//     type Type = &'a RoleType;
// }

// declare_storage_trait!(AccessControlEnumerableStorage);

// impl<T> AccessControlRoleManager for T
// where
//     T: AccessControlEnumerableStorage<Data = AccessControlEnumerableData> +
//     AccessControlStorage<Data = AccessControlData> +
//     Flush
// {
//     fn _grant_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
//         default_grant_role(self, role, account)?;
//         self._add(&role, &account)?;
//         Ok(())
//     }

//     fn _revoke_role(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
//         default_revoke_role(self, role, account)?;
//         self._remove(&role, &account)?;
//         Ok(())
//     }
// }

// pub trait AccessControlEnumerableInternal {
//     fn _enumerable(&self) -> &AccessControlEnumerableData;

//     fn _enumerable_mut(&mut self) -> &mut AccessControlEnumerableData;

//     fn _add(&mut self, member: &RoleType, value: &AccountId) -> Result<(), AccessControlError>;

//     fn _remove(&mut self, member: &RoleType, value: &AccountId) -> Result<(), AccessControlError>;

//     fn length(&self, member: &RoleType) -> u128;

//     fn _get_values(&self, member: &RoleType) -> Vec<AccountId>;
// }

// impl<T> AccessControlEnumerableInternal for T
// where
//     T: AccessControlEnumerableStorage<Data = AccessControlEnumerableData> +
//     AccessControlStorage<Data = AccessControlData> +
//     Flush
// {
//     #[inline(always)]
//     fn _enumerable(&self) -> &AccessControlEnumerableData {
//         AccessControlEnumerableStorage::get(self)
//     }

//     #[inline(always)]
//     fn _enumerable_mut(&mut self) -> &mut AccessControlEnumerableData {
//         AccessControlEnumerableStorage::get_mut(self)
//     }

//     fn _add(&mut self, member: &RoleType, value: &AccountId) -> Result<(), AccessControlError> {
//         match self._enumerable().role_members.get_index(member, value) {
//             None => {
//                 let mut values = self._enumerable().values.get(member).unwrap_or(Vec::<AccountId>::new());
//                 values.push(*value);
//                 self._enumerable_mut().values.insert(member, &values);
//                 self._enumerable_mut().role_members.insert(member, value);
//                 Ok(())
//             }
//             Some(_) => Err(AccessControlError::ValueAlreadyExists),
//         }
//     }

//     fn _remove(&mut self, member: &RoleType, value: &AccountId) -> Result<(), AccessControlError> {
//         let value_index = self
//             ._enumerable()
//             .role_members
//             .get_index(member, value)
//             .ok_or(AccessControlError::ValueNotExists)?;
//         let last_index = self.length(member) - 1;

//         if last_index != value_index {
//             let values = self._get_values(member);
//             let last_value = values
//                 .get(last_index as usize)
//                 .ok_or(AccessControlError::ValueNotExists)?;

//             let mut values = self._get_values(member);
//             let value = values
//                 .get_mut(value_index as usize)
//                 .ok_or(AccessControlError::ValueNotExists)?;
//             *value = *last_value;
//             self._enumerable_mut().values.insert(member, &values);
//         }

//         let mut values = self._get_values(member);
//         values.remove(last_index as usize);

//         self._enumerable_mut().values.insert(member, &values);
//         self._enumerable_mut().role_members.remove_value(member, value);
//         Ok(())
//     }

//     fn length(&self, member: &RoleType) -> u128 {
//         self._get_values(member).len() as u128
//     }

//     fn _get_values(&self, member: &RoleType) -> Vec<AccountId> {
//         self._enumerable()
//             .values
//             .get(&member)
//             .unwrap_or(Vec::<AccountId>::new())
//     }
// }

// impl<T> AccessControlEnumerable for T
// where
//     T: AccessControlEnumerableStorage<Data = AccessControlEnumerableData> +
//     AccessControlStorage<Data = AccessControlData> +
//     Flush
// {
//     default fn get_role_member(&self, role: RoleType, index: u128) -> Result<AccountId, AccessControlError> {
//         self._enumerable()
//             .role_members
//             .get_value(&role, &index)
//             .ok_or(AccessControlError::ValueNotExists)
//     }

//     default fn get_role_member_count(&self, role: RoleType) -> u128 {
//         self.length(&role) as u128
//     }
// }
