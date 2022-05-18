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

/// Extension of [`PSP35`] that allows token holders to destroy their tokens
use crate::traits::psp35::Id;
use crate::traits::psp35::PSP35Error;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type PSP35BatchRef = dyn PSP35Batch;

#[brush::trait_definition]
pub trait PSP35Batch {
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>, data: Vec<u8>) -> Result<(), PSP35Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), PSP35Error>;
}
