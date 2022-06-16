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

use ink_prelude::vec::Vec;
use ink_primitives::Key;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

use ink_storage::traits::{
    ExtKeyPtr,
    KeyPtr,
    PackedAllocate,
    PackedLayout,
    SpreadAllocate,
    SpreadLayout,
};

/// `Id` represents the identifier of the NFT. `Id::U8(1)` and `Id::U16(1)` are two different identifiers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

impl SpreadAllocate for Id
where
    Self: SpreadLayout,
{
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.next_for::<Id>();
        Id::U8(0)
    }
}
impl PackedAllocate for Id
where
    Self: PackedLayout,
{
    #[inline]
    fn allocate_packed(&mut self, _at: &Key) {}
}
