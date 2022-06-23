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
    traits::psp35::extensions::batch::*,
};

use openbrush::traits::{
    AccountId,
    AccountIdExt,
    Balance,
    Flush,
};

use crate::psp35::BalancesManager;
use ink_prelude::vec::Vec;

impl<B, T> PSP35Batch for T
where
    B: BalancesManager,
    T: PSP35Storage<Data = PSP35Data<B>> + Flush,
{
    default fn batch_transfer(
        &mut self,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error> {
        self._batch_transfer_from(Self::env().caller(), to, ids_amounts, data)
    }

    default fn batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error> {
        self._batch_transfer_from(from, to, ids_amounts, data)
    }
}

pub trait PSP35BatchInternal {
    fn _batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error>;
}

impl<B, T> PSP35BatchInternal for T
where
    B: BalancesManager,
    T: PSP35Storage<Data = PSP35Data<B>> + PSP35Internal + Flush,
{
    default fn _batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error> {
        let operator = Self::env().caller();

        for (id, value) in &ids_amounts {
            if to.is_zero() {
                return Err(PSP35Error::TransferToZeroAddress)
            }

            if from != operator && &self._get_allowance(&from, &operator, &Some(id)) < value {
                return Err(PSP35Error::NotAllowed)
            }
        }

        self._before_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        for (id, value) in &ids_amounts {
            self._decrease_allowance(&from, &operator, id, value.clone())?;

            self.get_mut().balances.decrease_balance(&from, id, value, false)?;
        }

        self._do_safe_transfer_check(&operator, &from, &to, &ids_amounts, &data)?;

        for (id, value) in &ids_amounts {
            self.get_mut().balances.increase_balance(&to, id, value, false)?;
        }

        self._after_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        self._emit_transfer_batch_event(Some(from), Some(to), ids_amounts);

        Ok(())
    }
}
