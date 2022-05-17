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
    psp35::*,
    traits::psp35::extensions::metadata::*,
};
use brush::declare_storage_trait;
pub use derive::PSP35MetadataStorage;
use ink_prelude::string::String;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP35MetadataData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP35MetadataData {
    pub uri: Option<String>,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP35MetadataStorage, PSP35MetadataData);

impl<T: PSP35MetadataStorage> PSP35Metadata for T {
    default fn uri(&self, _id: Id) -> Option<String> {
        self.get().uri.clone()
    }
}
