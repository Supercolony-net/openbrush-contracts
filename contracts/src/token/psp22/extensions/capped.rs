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
    traits::psp22::extensions::capped::*,
};
pub use derive::PSP22CappedStorage;
use ink_prelude::string::String;
use openbrush::{
    declare_storage_trait,
    traits::{
        Balance,
        InkStorage,
    },
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP22CappedData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct PSP22CappedData {
    pub cap: Balance,
    pub _reserved: Option<()>,
}

declare_storage_trait!(PSP22CappedStorage);

impl<T: PSP22CappedStorage<Data = PSP22CappedData> + PSP22CappedInternal + PSP22Internal + InkStorage> PSP22Capped
    for T
{
    default fn cap(&self) -> Balance {
        self.get().cap
    }
}

pub trait PSP22CappedInternal {
    /// Check for cap overflow before minting tokens
    fn _before_mint(&self, amount: Balance) -> Result<(), PSP22Error>;

    /// Initializes the token's cap
    fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error>;
}

impl<T: PSP22CappedStorage<Data = PSP22CappedData> + PSP22> PSP22CappedInternal for T {
    fn _before_mint(&self, amount: Balance) -> Result<(), PSP22Error> {
        if (self.total_supply() + amount) > self.get().cap {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        Ok(())
    }

    fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
        if cap == 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
        }
        PSP22CappedStorage::get_mut(self).cap = cap;
        Ok(())
    }
}
