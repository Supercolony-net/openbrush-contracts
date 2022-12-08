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
    psp37,
    psp37::{
        balances,
        extensions::enumerable,
    },
    traits::psp37::{
        extensions::enumerable::*,
        *,
    },
};
use ink::storage::traits::ManualKey;

use openbrush::{
    storage::{
        Mapping,
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
pub use psp37::{
    Internal as _,
    Transfer as _,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Balances);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Balances {
    pub enumerable: MultiMapping<Option<AccountId>, Id, ManualKey<{ STORAGE_KEY + 1 }>, EnumerableKey>,
    pub balances: Mapping<(AccountId, Id), Balance, ManualKey<{ STORAGE_KEY + 2 }>, BalancesKey>,
    pub supply: Mapping<Id, Balance, ManualKey<{ STORAGE_KEY + 3 }>>,
    pub _reserved: Option<()>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub struct BalancesKey;

impl<'a> TypeGuard<'a> for BalancesKey {
    type Type = &'a (&'a AccountId, &'a Id);
}

impl balances::BalancesManager for Balances {
    #[inline(always)]
    fn balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
        match id {
            None => self.enumerable.count(&Some(owner)),
            Some(id) => self.balances.get(&(owner, id)).unwrap_or(0),
        }
    }

    #[inline(always)]
    fn total_supply(&self, id: &Option<&Id>) -> Balance {
        match id {
            None => self.enumerable.count(&None),
            Some(id) => self.supply.get(id).unwrap_or(0),
        }
    }

    fn increase_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, mint: bool) -> Result<(), PSP37Error> {
        let amount = *amount;

        if amount == 0 {
            return Ok(())
        }

        let balance_before = self.balance_of(owner, &Some(id));
        self.balances
            .insert(&(owner, id), &(balance_before.checked_add(amount).unwrap()));

        if balance_before == 0 {
            self.enumerable.insert(&Some(owner), id);
        }

        if mint {
            let supply_before = self.total_supply(&Some(id));

            self.supply.insert(id, &(supply_before.checked_add(amount).unwrap()));

            if supply_before == 0 {
                self.enumerable.insert(&None, id);
            }
        }
        Ok(())
    }

    fn decrease_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, burn: bool) -> Result<(), PSP37Error> {
        let amount = *amount;

        if amount == 0 {
            return Ok(())
        }

        let balance_after = self
            .balance_of(owner, &Some(id))
            .checked_sub(amount)
            .ok_or(PSP37Error::InsufficientBalance)?;
        self.balances.insert(&(owner, id), &balance_after);

        if balance_after == 0 {
            self.enumerable.remove_value(&Some(owner), id);
        }

        if burn {
            let supply_after = self
                .total_supply(&Some(id))
                .checked_sub(amount)
                .ok_or(PSP37Error::InsufficientBalance)?;
            self.supply.insert(id, &supply_after);

            if supply_after == 0 {
                self.enumerable.remove_value(&None, id);
            }
        }
        Ok(())
    }
}

impl<T> PSP37Enumerable for T
where
    T: Storage<psp37::Data<Balances>>,
    T: OccupiedStorage<{ psp37::STORAGE_KEY }, WithData = psp37::Data<Balances>>,
{
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
        self.data().balances.enumerable.get_value(&Some(&owner), &index)
    }

    default fn token_by_index(&self, index: u128) -> Option<Id> {
        self.data().balances.enumerable.get_value(&None, &index)
    }
}
