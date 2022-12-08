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
    access_control::{
        extensions::enumerable,
        members,
    },
    traits::access_control::{
        extensions::enumerable::*,
        *,
    },
};
pub use access_control::Internal as _;
use ink::storage::traits::ManualKey;

use openbrush::{
    storage::{
        MultiMapping,
        ValueGuard,
    },
    traits::{
        AccountId,
        OccupiedStorage,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Members);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Members {
    pub role_members: MultiMapping<RoleType, AccountId, ManualKey<{ STORAGE_KEY + 1 }>, ValueGuard<RoleType>>,
    pub _reserved: Option<()>,
}

impl members::MembersManager for Members {
    fn has_role(&self, role: RoleType, address: &AccountId) -> bool {
        self.role_members.contains_value(role, address)
    }

    fn add(&mut self, role: RoleType, member: &AccountId) {
        self.role_members.insert(role, member);
    }

    fn remove(&mut self, role: RoleType, member: &AccountId) {
        self.role_members.remove_value(role, member);
    }
}

impl<T> AccessControlEnumerable for T
where
    T: Storage<access_control::Data<Members>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<Members>>,
{
    default fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId> {
        self.data().members.role_members.get_value(role, &(index as u128))
    }

    default fn get_role_member_count(&self, role: RoleType) -> u32 {
        self.data().members.role_members.count(role) as u32
    }
}
