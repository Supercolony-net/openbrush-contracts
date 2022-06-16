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
    psp22::*,
    traits::psp22::extensions::metadata::*,
};
pub use derive::PSP22MetadataStorage;
use ink_prelude::string::String;
use openbrush::declare_storage_trait;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP22MetadataData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP22MetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP22MetadataStorage);

impl<T: PSP22MetadataStorage<Data = PSP22MetadataData>> PSP22Metadata for T {
    default fn token_name(&self) -> Option<String> {
        self.get().name.clone()
    }

    default fn token_symbol(&self) -> Option<String> {
        self.get().symbol.clone()
    }

    default fn token_decimals(&self) -> u8 {
        self.get().decimals.clone()
    }
}
