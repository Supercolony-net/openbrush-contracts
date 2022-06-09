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

impl SpreadAllocate for Id {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.next_for::<Id>();
        Id::U8(0)
    }
}
impl PackedAllocate for Id {
    #[inline]
    fn allocate_packed(&mut self, _at: &Key) {}
}
