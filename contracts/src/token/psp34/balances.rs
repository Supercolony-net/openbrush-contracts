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

use crate::psp34::{
    Id,
    Owner,
};
use openbrush::{
    storage::Mapping,
    traits::Balance,
};

pub const BALANCES_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP34Balances");

pub trait BalancesManager {
    fn balance_of(&self, owner: &Owner) -> u32;
    fn increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool);
    fn decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool);
    fn total_supply(&self) -> Balance;
}

#[derive(Default, Debug)]
#[openbrush::storage(BALANCES_KEY)]
pub struct Balances {
    owned_tokens_count: Mapping<Owner, u32>,
    total_supply: Balance,
}

impl BalancesManager for Balances {
    #[inline(always)]
    fn balance_of(&self, owner: &Owner) -> u32 {
        self.owned_tokens_count.get(owner).unwrap_or(0)
    }

    #[inline(always)]
    fn increase_balance(&mut self, owner: &Owner, _id: &Id, increase_supply: bool) {
        let to_balance = self.owned_tokens_count.get(owner).unwrap_or(0);
        self.owned_tokens_count.insert(owner, &(to_balance + 1));
        if increase_supply {
            self.total_supply += 1;
        }
    }

    #[inline(always)]
    fn decrease_balance(&mut self, owner: &Owner, _id: &Id, decrease_supply: bool) {
        let from_balance = self.owned_tokens_count.get(owner).unwrap_or(0);
        self.owned_tokens_count
            .insert(owner, &(from_balance.checked_sub(1).unwrap()));

        if decrease_supply {
            self.total_supply -= 1;
        }
    }

    #[inline(always)]
    fn total_supply(&self) -> u128 {
        self.total_supply
    }
}
