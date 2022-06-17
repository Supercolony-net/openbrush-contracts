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

use crate::psp35::Id;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
    },
};

pub const BALANCES_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP35Balances");

pub trait BalancesManager {
    fn balance_of(&self, owner: &AccountId, id: &Id) -> Balance;
    fn mint(&mut self, owner: &AccountId, id: &Id, amount: Balance, increase_supply: bool);
    fn burn(&mut self, owner: &AccountId, id: &Id, amount: Balance, decrease_supply: bool);
}

#[derive(Default, Debug)]
#[openbrush::storage(BALANCES_KEY)]
pub struct Balances {
    balances: Mapping<(AccountId, Id), Balance>,
}

impl BalancesManager for Balances {
    #[inline(always)]
    fn balance_of(&self, owner: &AccountId, id: &Id) -> Balance {
        self.balances.get(&(owner.clone(), id.clone())).unwrap_or(0)
    }

    #[inline(always)]
    fn mint(&mut self, owner: &AccountId, id: &Id, amount: Balance, _increase_supply: bool) {
        let to_balance = self.balance_of(owner, id);
        self.balances
            .insert(&(owner.clone(), id.clone()), &(to_balance.checked_add(amount).unwrap()));
    }

    #[inline(always)]
    fn burn(&mut self, owner: &AccountId, id: &Id, amount: Balance, _decrease_supply: bool) {
        let from_balance = self.balance_of(owner, id);
        self.balances.insert(
            &(owner.clone(), id.clone()),
            &(from_balance.checked_sub(amount).unwrap()),
        );
    }
}
