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
    psp34::*,
    traits::psp34::extensions::metadata::*,
};
pub use derive::{
    PSP34MetadataStorage,
    PSP34Storage,
};
use ink_prelude::vec::Vec;
use openbrush::{
    declare_storage_trait,
    storage::{
        Mapping,
        TypeGuard,
    },
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP32MetadataData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP34MetadataData {
    pub attributes: Mapping<(Id, Vec<u8>), Vec<u8>, AttributesKey>,
    pub _reserved: Option<()>,
}

pub struct AttributesKey;

impl<'a> TypeGuard<'a> for AttributesKey {
    type Type = &'a (&'a Id, &'a Vec<u8>);
}

declare_storage_trait!(PSP34MetadataStorage);

impl<T: PSP34MetadataStorage<Data = PSP34MetadataData>> PSP34Metadata for T {
    default fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        self.get().attributes.get(&(&id, &key))
    }
}

pub trait PSP34MetadataInternal {
    fn _set_attribute(&mut self, id: Id, key: Vec<u8>, value: Vec<u8>);
}

impl<T: PSP34MetadataStorage<Data = PSP34MetadataData> + PSP34Internal> PSP34MetadataInternal for T {
    default fn _set_attribute(&mut self, id: Id, key: Vec<u8>, value: Vec<u8>) {
        self.get_mut().attributes.insert(&(&id, &key), &value);
        self._emit_attribute_set_event(id, key, value);
    }
}
