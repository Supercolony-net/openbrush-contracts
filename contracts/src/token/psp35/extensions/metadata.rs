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
    psp35,
    psp35::extensions::metadata,
    traits::psp35::{
        extensions::metadata::*,
        *,
    },
};
pub use psp35::Internal as _;

use ink_prelude::vec::Vec;
use ink_storage::Mapping;
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct Data {
    pub attributes: Mapping<(Id, Vec<u8>), Vec<u8>>,
    pub _reserved: Option<()>,
}

impl<T: Storage<Data>> PSP35Metadata for T {
    default fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        self.data().attributes.get(&(id, key))
    }
}

pub trait Internal {
    fn _emit_attribute_set_event(&self, _id: &Id, _key: &Vec<u8>, _data: &Vec<u8>);

    fn _set_attribute(&mut self, id: &Id, key: &Vec<u8>, data: &Vec<u8>) -> Result<(), PSP35Error>;

    fn _get_attribute(&self, id: &Id, key: &Vec<u8>) -> Option<Vec<u8>>;
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_attribute_set_event(&self, _id: &Id, _key: &Vec<u8>, _data: &Vec<u8>) {}

    default fn _set_attribute(&mut self, id: &Id, key: &Vec<u8>, data: &Vec<u8>) -> Result<(), PSP35Error> {
        self.data().attributes.insert((id, key), data);
        self._emit_attribute_set_event(id, key, data);
        Ok(())
    }

    default fn _get_attribute(&self, id: &Id, key: &Vec<u8>) -> Option<Vec<u8>> {
        self.data().attributes.get((id, key))
    }
}
