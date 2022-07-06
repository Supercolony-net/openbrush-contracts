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
        extensions::enumerable,
    },
    traits::psp34::{
        extensions::enumerable::*,
        *,
    },
};
pub use psp34::{
    Internal as _,
    Transfer as _,
};

use crate::psp34::Owner;
use openbrush::{
    storage::{
        MultiMapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        OccupiedStorage,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Balances);

#[derive(Default, Debug)]
#[openbrush::upgradable_storage(STORAGE_KEY)]
pub struct Balances {
    pub enumerable: MultiMapping<Option<AccountId>, Id, EnumerableKey /* optimization */>,
    pub _reserved: Option<()>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

impl balances::BalancesManager for Balances {
    fn balance_of(&self, owner: &Owner) -> u32 {
        self.enumerable.count(&Some(owner)) as u32
    }

    fn increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
        self.enumerable.insert(&Some(owner), id);
        if increase_supply {
            self.enumerable.insert(&None, id);
        }
    }

    fn decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
        self.enumerable.remove_value(&Some(owner), id);
        if decrease_supply {
            self.enumerable.remove_value(&None, id);
        }
    }

    fn total_supply(&self) -> Balance {
        self.enumerable.count(&None)
    }
}

impl<T> PSP34Enumerable for T
where
    T: Storage<psp34::Data<Balances>>,
    T: OccupiedStorage<{ psp34::STORAGE_KEY }, WithData = psp34::Data<Balances>>,
{
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        self.data()
            .balances
            .enumerable
            .get_value(&Some(&owner), &index)
            .ok_or(PSP34Error::TokenNotExists)
    }

    default fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        self.data()
            .balances
            .enumerable
            .get_value(&None, &index)
            .ok_or(PSP34Error::TokenNotExists)
    }
}
