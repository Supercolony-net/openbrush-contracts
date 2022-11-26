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

pub use crate::traits::access_control::*;
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::AccountId,
};
use scale::{
    Error,
    Input,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Members);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Members {
    pub members: Mapping<(RoleType, AccountId), (), MembersKey>,
    pub _reserved: Option<()>,
}

pub struct MembersKey;

impl<'a> TypeGuard<'a> for MembersKey {
    type Type = &'a (RoleType, &'a AccountId);
}

pub trait MembersManager {
    fn has_role(&self, role: RoleType, address: &AccountId) -> bool;

    fn add(&mut self, role: RoleType, member: &AccountId);

    fn remove(&mut self, role: RoleType, member: &AccountId);
}

impl MembersManager for Members {
    fn has_role(&self, role: RoleType, address: &AccountId) -> bool {
        self.members.contains(&(role, address))
    }

    fn add(&mut self, role: RoleType, member: &AccountId) {
        self.members.insert(&(role, member), &());
    }

    fn remove(&mut self, role: RoleType, member: &AccountId) {
        self.members.remove(&(role, member));
    }
}
