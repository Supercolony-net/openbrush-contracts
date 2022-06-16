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

pub use crate::traits::errors::ReentrancyGuardError;
pub use derive::ReentrancyGuardStorage;
use ink_storage::traits::push_spread_root;
use openbrush::{
    declare_storage_trait,
    modifier_definition,
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::ReentrancyGuardData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct ReentrancyGuardData {
    pub status: u8,
    pub _reserved: Option<()>,
}

declare_storage_trait!(ReentrancyGuardStorage);

const NOT_ENTERED: u8 = 0;
const ENTERED: u8 = 1;

/// Prevents a contract from calling itself, directly or indirectly.
/// Calling a `non_reentrant` function from another `non_reentrant`
/// function is not supported. It is possible to prevent this from happening
/// by making the `non_reentrant` function external, and make it call a
/// `private` function that does the actual work.
///
/// This modifier flushes the struct into storage with `ENTERED`
/// status before calling the original method.
#[modifier_definition]
pub fn non_reentrant<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: ReentrancyGuardStorage<Data = ReentrancyGuardData>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<ReentrancyGuardError>,
{
    if instance.get().status == ENTERED {
        return Err(From::from(ReentrancyGuardError::ReentrantCall))
    }
    // Any calls to nonReentrant after this point will fail
    instance.get_mut().status = ENTERED;

    // We want to flush storage before execution of inner function,
    // because ink! doesn't do it by default and `status` will not be updated in child calls
    push_spread_root(instance.get(), &Default::default());

    let result = body(instance);
    instance.get_mut().status = NOT_ENTERED;

    return result
}
