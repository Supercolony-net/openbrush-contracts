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

use crate::psp35::{
    Id,
    PSP35Error,
};
use ink_storage::traits::{
    SpreadAllocate,
    SpreadLayout,
};
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
    },
};

pub const BALANCES_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP35Balances");

pub trait BalancesManager: SpreadLayout + SpreadAllocate {
    fn balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance;
    fn total_supply(&self, id: &Option<&Id>) -> Balance;
    fn increase_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, mint: bool) -> Result<(), PSP35Error>;
    fn decrease_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, burn: bool) -> Result<(), PSP35Error>;
}

#[derive(Default, Debug)]
#[openbrush::storage(BALANCES_KEY)]
pub struct Balances {
    pub balances: Mapping<(AccountId, Option<Id>), Balance, BalancesKey>,
    pub supply: Mapping<Option<Id>, Balance, SupplyKey>,
    pub _reserved: Option<()>,
}

pub struct BalancesKey;

impl<'a> TypeGuard<'a> for BalancesKey {
    type Type = &'a (&'a AccountId, &'a Option<&'a Id>);
}

pub struct SupplyKey;

impl<'a> TypeGuard<'a> for SupplyKey {
    type Type = &'a Option<&'a Id>;
}

impl BalancesManager for Balances {
    #[inline(always)]
    fn balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
        self.balances.get(&(owner, id)).unwrap_or(0)
    }

    #[inline(always)]
    fn total_supply(&self, id: &Option<&Id>) -> Balance {
        self.supply.get(id).unwrap_or(0)
    }

    fn increase_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, mint: bool) -> Result<(), PSP35Error> {
        let amount = *amount;

        if amount == 0 {
            return Ok(())
        }

        let id = &Some(id);
        let balance_before = self.balance_of(owner, id);

        if balance_before == 0 {
            self.balances
                .insert(&(owner, &None), &self.balance_of(owner, &None).checked_add(1).unwrap());
        }

        self.balances
            .insert(&(owner, id), &balance_before.checked_add(amount).unwrap());

        if mint {
            let supply_before = self.total_supply(id);
            self.supply.insert(id, &supply_before.checked_add(amount).unwrap());

            if supply_before == 0 {
                self.supply
                    .insert(&None, &self.total_supply(&None).checked_add(1).unwrap());
            }
        }

        Ok(())
    }

    fn decrease_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, burn: bool) -> Result<(), PSP35Error> {
        let amount = *amount;

        if amount == 0 {
            return Ok(())
        }

        let id = &Some(id);
        let balance_after = self
            .balance_of(owner, id)
            .checked_sub(amount)
            .ok_or(PSP35Error::InsufficientBalance)?;
        self.balances.insert(&(owner, id), &balance_after);

        if balance_after == 0 {
            self.balances.insert(
                &(owner, &None),
                &self
                    .balance_of(owner, &None)
                    .checked_sub(1)
                    .ok_or(PSP35Error::InsufficientBalance)?,
            );
        }

        if burn {
            let supply_after = self
                .total_supply(id)
                .checked_sub(amount)
                .ok_or(PSP35Error::InsufficientBalance)?;
            self.supply.insert(id, &supply_after);

            if supply_after == 0 {
                self.supply.insert(
                    &None,
                    &self
                        .total_supply(&None)
                        .checked_sub(1)
                        .ok_or(PSP35Error::InsufficientBalance)?,
                );
            }
        }
        Ok(())
    }
}
