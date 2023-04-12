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
    psp34,
    psp34::{
        balances,
        extensions::metadata,
    },
    traits::psp34::{
        extensions::metadata::*,
        *,
    },
};

pub use metadata::Internal as _;
pub use psp34::{
    Internal as _,
    Transfer as _,
};

use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        Storage,
        String
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub attributes: Mapping<(Id, String), String, AttributesKey>,
    pub _reserved: Option<()>,
}

pub struct AttributesKey;

impl<'a> TypeGuard<'a> for AttributesKey {
    type Type = &'a (&'a Id, &'a String);
}

impl<T: Storage<Data>> PSP34Metadata for T {
    default fn get_attribute(&self, id: Id, key: String) -> Option<String> {
        self.data().attributes.get(&(&id, &key))
    }
}

pub trait Internal {
    /// Event is emitted when an attribute is set for a token.
    fn _emit_attribute_set_event(&self, _id: Id, _key: String, _data: String);

    fn _set_attribute(&mut self, id: Id, key: String, value: String);
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    default fn _emit_attribute_set_event(&self, _id: Id, _key: String, _data: String) {}

    default fn _set_attribute(&mut self, id: Id, key: String, value: String) {
        self.data().attributes.insert(&(&id, &key), &value);
        self._emit_attribute_set_event(id, key, value);
    }
}
