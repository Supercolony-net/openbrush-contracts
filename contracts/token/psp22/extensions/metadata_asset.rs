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
    psp22::utils::pallet_assets::*
};
use brush::declare_storage_trait;
pub use derive::PSP22AssetMetadataStorage;
use ink_prelude::string::String;

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("brush::PSP22AssetMetadataData");

#[derive(Default, Debug)]
#[brush::storage(STORAGE_KEY)]
pub struct PSP22AssetMetadataData {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
    // pub _reserved: Option<()>,
}

declare_storage_trait!(PSP22AssetMetadataStorage, PSP22AssetMetadataData);

impl<T: PSP22AssetMetadataStorage> PSP22AssetMetadata for T {
    default fn token_name(&self) -> Option<String> {
        self.get().name.clone()
    }

    default fn token_symbol(&self) -> Option<String> {
        // self.get().symbol.clone()
    }

    default fn token_decimals(&self) -> u8 {
        // self.get().decimals.clone()
    }
}

pub trait PSP22AssetMetadataInternal {
    fn _set_metadata(&self, name : Option<String>, symbol : Option<String>, decimals : u8)-> Result<(), PSP22Error>;
}

impl<T: PSP22AssetStorage> PSP22AssetMetadataInternal for T{
    default fn _set_metadata(&self, name : Option<String>, symbol : Option<String>, decimals : u8)-> Result<(), PSP22Error> {
        // unimplemented!()
        PalletAsset::_set_metadata(self.get().asset_id, vec![1u8;1], vec![1u8;1], decimals).unwrap()
    }
}